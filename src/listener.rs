use bstr::BStr;
use eyre::Context;
use fossbeamer::{Command, Info};
use parking_lot::RwLock;
use rumqttc::{Client, MqttOptions, Packet, Publish};
use std::{
    collections::HashMap,
    sync::{mpsc::Sender, Arc},
    thread,
    time::Duration,
};
use tracing::{debug, info, warn, Span};

/// Maintains a connection to an MQTT broker.
pub(crate) struct MQTT {
    /// The MQTT client
    client: rumqttc::Client,

    /// The topic that's prepended before IDs in the topic
    topic_prefix: String,

    /// Senders expecting commands to be sent to, keyed by their topic.
    senders: Arc<RwLock<HashMap<String, Sender<fossbeamer::Command>>>>,
}

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

        let senders = Arc::new(RwLock::new(
            HashMap::<String, Sender<fossbeamer::Command>>::new(),
        ));

        let topic_prefix: String = topic_prefix.into();
        let catchall_topic = topic_prefix.clone();

        thread::spawn({
            let senders = senders.clone();
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

                                // parse the command
                                let command = match serde_json::from_slice::<Command>(&payload) {
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
                                    for (_topic, sender) in senders.read().iter() {
                                        if let Err(e) = sender.send(command.clone()) {
                                            warn!(err=%e, "unable to send command to tx");
                                        }
                                    }
                                } else {
                                    match senders.read().get(&topic) {
                                        None => {
                                            warn!("couldn't find topic");
                                            continue;
                                        }
                                        Some(tx) => {
                                            if let Err(e) = tx.send(command) {
                                                warn!(err=%e, "unable to send command to tx");
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
        client
            .subscribe(catchall_topic, rumqttc::QoS::AtLeastOnce)
            .context("subscribing to catchall topic")?;

        Ok(Self {
            client,
            senders,
            topic_prefix,
        })
    }

    /// Register a new display, using the passed display_info.
    /// `set` requests received are sent to the passed channel.
    pub fn add_display(
        &self,
        display_info: &Info,
        tx: Sender<fossbeamer::Command>,
    ) -> eyre::Result<()> {
        let k = &display_info.serial;
        let topic_str = format!("{}/{}", self.topic_prefix, k);

        self.client
            .subscribe(&topic_str, rumqttc::QoS::AtLeastOnce)
            .context("subscribing to topic")?;

        self.senders.write().insert(topic_str, tx);

        Ok(())
    }
}
