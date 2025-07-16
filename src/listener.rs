use bstr::BStr;
use eyre::Context;
use fossbeamer::display;
use parking_lot::RwLock;
use rumqttc::{Client, MqttOptions, Packet, Publish};
use std::{collections::HashMap, sync::Arc, thread, time::Duration};
use tracing::{debug, info, warn, Span};

/// Maintains a connection to an MQTT broker.
pub(crate) struct MQTT {
    /// The MQTT client
    client: rumqttc::Client,

    /// The topic that's prepended before IDs in the topic
    topic_prefix: String,

    /// Keep track of all displays
    displays: Arc<RwLock<HashMap<String, DisplayHandle>>>,
}

type DisplayHandle = Box<dyn display::Display>;

impl MQTT {
    /// Prepares a connection to the broker, and spawns off a thread dealing
    /// with received messages.
    /// It spawns off a thread relaying messages to the Senders added in a
    /// [add_display] call.
    pub fn new(
        id: impl Into<String>,
        host: impl Into<String>,
        port: u16,
        topic_prefix: impl Into<String> + Clone,
    ) -> eyre::Result<Self> {
        let (client, mut connection) = Client::new(MqttOptions::new(id, host, port), 64);

        let displays = Arc::new(RwLock::new(HashMap::<String, DisplayHandle>::new()));

        let topic_prefix: String = topic_prefix.into();
        let catchall_topic = topic_prefix.clone();

        thread::spawn({
            let displays = displays.clone();
            let catchall_topic = catchall_topic.clone();
            move || {
                for event in connection.iter() {
                    match event {
                        Ok(event) => match event {
                            rumqttc::Event::Incoming(Packet::Publish(Publish {
                                topic,
                                payload,
                                ..
                            })) => {
                                Span::current().record("topic", &topic);

                                // parse the scenario
                                let scenario = match serde_json::from_slice::<display::Scenario>(
                                    &payload,
                                ) {
                                    Ok(command) => {
                                        info!(?command, "received command");
                                        command
                                    }
                                    Err(e) => {
                                        warn!(err=%e, payload=%BStr::new(&payload), "received payload that couldn't be parsed");
                                        continue;
                                    }
                                };

                                if topic == catchall_topic {
                                    for (_topic, display) in displays.read().iter() {
                                        if let Err(err) = display.run_scenario(scenario.clone()) {
                                            warn!(%err, "unable to run scenario");
                                        }
                                    }
                                } else {
                                    match displays.read().get(&topic) {
                                        None => {
                                            warn!("couldn't find topic");
                                            continue;
                                        }
                                        Some(display) => {
                                            if let Err(err) = display.run_scenario(scenario) {
                                                warn!(%err, "unable to run scenario");
                                            }
                                        }
                                    }
                                }
                            }
                            rumqttc::Event::Incoming(incoming) => {
                                debug!(?incoming, "other incoming event");
                            }
                            rumqttc::Event::Outgoing(out) => {
                                debug!(?out, "outgoing event");
                            }
                        },
                        Err(e) => {
                            warn!(err=%e, "connection error");
                            // sleep a bit
                            std::thread::sleep(Duration::from_secs(5));
                        }
                    }
                }
            }
        });

        // subscribe to the catchall
        info!(topic=%catchall_topic, "subscribing to catchall topic");
        client
            .subscribe(catchall_topic, rumqttc::QoS::AtLeastOnce)
            .context("subscribing to catchall topic")?;

        Ok(Self {
            client,
            displays,
            topic_prefix,
        })
    }

    /// Register a new display, using the passed display_info.
    /// `set` requests received are sent to the passed channel.
    pub fn add_display<D>(&self, display: D, display_info: &display::Info) -> eyre::Result<()>
    where
        D: display::Display + Send + 'static,
    {
        let k = &display_info.serial;
        let topic = format!("{}/{}", self.topic_prefix, k);

        info!(topic, "subscribing to topic");
        self.client
            .subscribe(&topic, rumqttc::QoS::AtLeastOnce)
            .context("subscribing to topic")?;

        self.displays.write().insert(topic, Box::new(display));

        Ok(())
    }
}
