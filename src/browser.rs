use eyre::Context;
use std::{sync::mpsc, thread};
use tao::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    platform::unix::EventLoopBuilderExtUnix,
    window::WindowBuilder,
};
use tracing::{debug, warn};
use wry::WebViewBuilder;

use crate::display::{self, Scenario};

pub struct BrowserWindow {
    tx: mpsc::SyncSender<Command>,
    display_info: display::Info,
}

impl BrowserWindow {
    pub fn new(display_info: display::Info) -> Self {
        let (tx, rx) = mpsc::sync_channel(0);

        // spawn a thread receiving scenarios and process them.
        thread::spawn(move || Self::run(rx).unwrap());

        Self { tx, display_info }
    }

    /// Opens a browser window, and processes [Command] events sent to it.
    // TODO: We should rearrange the logic, we want the main thread to be the event loop in the end.
    fn run(commands: mpsc::Receiver<Command>) -> wry::Result<()> {
        let event_loop = EventLoopBuilder::<Command>::with_user_event()
            .with_any_thread(true) // HACK
            .build();

        let window = WindowBuilder::new()
            .with_title("Fossbeamer")
            .build(&event_loop)
            .unwrap();

        #[cfg(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "ios",
            target_os = "android"
        ))]
        let builder = WebViewBuilder::new(&window);

        #[cfg(not(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "ios",
            target_os = "android"
        )))]
        let builder = {
            use tao::platform::unix::WindowExtUnix;
            use wry::WebViewBuilderExtUnix;
            let vbox = window.default_vbox().unwrap();
            WebViewBuilder::new_gtk(vbox)
        };

        let webview = builder.with_url("about:blank").build()?;

        let proxy = event_loop.create_proxy();
        thread::spawn(move || {
            while let Ok(command) = commands.recv() {
                proxy.send_event(command).unwrap();
            }
        });

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            debug!(?event, "got event");

            match event {
                Event::NewEvents(StartCause::Init) => debug!("init"),
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                }
                | Event::UserEvent(Command::Stop) => *control_flow = ControlFlow::Exit,
                Event::UserEvent(Command::LoadUrl { url }) => {
                    if let Err(e) = webview.load_url(url.as_str()) {
                        warn!(err=%e, "unable to load webview")
                    };
                }
                _ => {
                    debug!(?event, "got other event")
                }
            }
        })
    }
}

impl Drop for BrowserWindow {
    fn drop(&mut self) {
        self.tx.send(Command::Stop).unwrap();
    }
}

impl crate::display::Display for BrowserWindow {
    fn run_scenario(&self, scenario: Scenario) -> eyre::Result<()> {
        match scenario {
            Scenario::URL { url } => self
                .tx
                .send(Command::LoadUrl { url })
                .wrap_err("sending command"),
            Scenario::Blank => self
                .tx
                .send(Command::LoadUrl {
                    url: "about:blank".to_string(),
                })
                .wrap_err("sending command"),
            Scenario::Video { url: _ } => Err(eyre::eyre!("video unimplemented so far")),
        }
    }

    fn get_info(&self) -> &display::Info {
        &self.display_info
    }
}

/// Commands to control [BrowserWindow].
#[derive(Clone, Debug)]
enum Command {
    LoadUrl { url: String },
    Stop,
}
