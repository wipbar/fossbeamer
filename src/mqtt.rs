use std::{sync::mpsc::Sender, thread};

use rumqttc::{Client, ClientError, MqttOptions, Packet, Publish};

use crate::common::Command;

pub(crate) struct Listener {
    pub id: String,
    pub host: String,
    pub port: u16,
    pub sender: Sender<Command>,
}

impl Listener {
    pub(crate) fn start(self) -> Result<(), ClientError> {
        let (client, mut connection) =
            Client::new(MqttOptions::new(&self.id, self.host, self.port), 64);

        client.subscribe("screens", rumqttc::QoS::AtLeastOnce)?;
        client.subscribe(format!("screens/{}", self.id), rumqttc::QoS::AtLeastOnce)?;

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
