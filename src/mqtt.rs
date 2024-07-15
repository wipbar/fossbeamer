use std::{sync::mpsc::Sender, thread, time::Duration};

use fossbeamer::Command;
use rumqttc::{Client, ClientError, MqttOptions, Packet, Publish};
use tracing::{debug, warn};

pub(crate) struct Listener {
    pub id: String,
    pub host: String,
    pub port: u16,
    pub sender: Sender<fossbeamer::Command>,
}

impl Listener {
    pub(crate) fn start(self) -> Result<(), ClientError> {
        let (client, mut connection) =
            Client::new(MqttOptions::new(&self.id, self.host, self.port), 64);

        client.subscribe("screens", rumqttc::QoS::AtLeastOnce)?;
        client.subscribe(format!("screens/{}", self.id), rumqttc::QoS::AtLeastOnce)?;

        thread::spawn(move || {
            for event in connection.iter() {
                match event {
                    Ok(event) => match event {
                        rumqttc::Event::Incoming(Packet::Publish(Publish {
                            topic,
                            payload,
                            ..
                        })) => {
                            if topic == "commands" {
                                if let Ok(command) = serde_json::from_slice::<Command>(&payload) {
                                    debug!(?command, "received command");

                                    self.sender.send(command).unwrap();
                                }
                            } else {
                                debug!(?topic, "received other topic");
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
        });

        Ok(())
    }
}
