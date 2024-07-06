use std::{sync::mpsc::Receiver, thread};

use serde::Deserialize;
use tao::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

#[derive(Debug, Deserialize)]
#[serde(tag = "kind")]
pub enum Command {
    LoadUrl { url: String },
    Reload,
    Stop,
}

pub fn spawn_browser(url: String, command_receiver: Option<Receiver<Command>>) -> wry::Result<()> {
    let event_loop = EventLoopBuilder::<Command>::with_user_event().build();
    let window = WindowBuilder::new()
        .with_title("BornScreen")
        .build(&event_loop)
        .unwrap();
    let webview = WebViewBuilder::new(&window).with_url(url).build()?;

    if let Some(command_receiver) = command_receiver {
        let proxy = event_loop.create_proxy();
        thread::spawn(move || {
            while let Ok(command) = command_receiver.recv() {
                // TODO: Remove the use of unwrap
                proxy.send_event(command).unwrap();
            }
        });
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Opened a browser window!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            }
            | Event::UserEvent(Command::Stop) => *control_flow = ControlFlow::Exit,
            Event::UserEvent(Command::Reload) => {
                if let Ok(url) = webview.url() {
                    // TODO: Remove the use of unwrap
                    webview.load_url(url.as_str()).unwrap();
                }
            }
            // TODO: Remove the use of unwrap
            Event::UserEvent(Command::LoadUrl { url }) => webview.load_url(url.as_str()).unwrap(),
            _ => (),
        }
    })
}
