use std::{sync::mpsc::Sender, thread};

use fossbeamer::Command;
use rumqttc::{Client, ClientError, MqttOptions, Packet, Publish};

pub(crate) struct Listener {
    pub id: String,
    pub host: String,
    pub port: u16,
    pub sender: Sender<fossbeamer::Command>,
}

impl Listener {
    pub(crate) fn start(self) -> Result<(), ClientError> {
        let (client, mut connection) =
            Client::new(MqttOptions::new(self.id, self.host, self.port), 64);

        client.subscribe("commands", rumqttc::QoS::AtLeastOnce)?;

        thread::spawn(move || {
            for event in connection.iter() {
                println!("{:?}", event);

                if let Ok(rumqttc::Event::Incoming(Packet::Publish(Publish {
                    topic,
                    payload,
                    ..
                }))) = event
                {
                    if topic == "commands" {
                        if let Ok(command) = serde_json::from_slice::<Command>(&payload) {
                            println!("{:?}", command);

                            self.sender.send(command).unwrap();
                        }
                    }
                }
            }
        });

        Ok(())
    }
}
