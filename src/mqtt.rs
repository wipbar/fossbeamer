use std::{sync::mpsc::Sender, thread};

use rumqttc::{Client, MqttOptions, Packet, Publish};

use crate::{
    common::{Command, Config},
    system::get_cpu_serial,
};

pub(crate) struct Listener<F: Fn() -> Config + std::marker::Send> {
    pub get_config: F,
    pub sender: Sender<Command>,
}

impl<F: Fn() -> Config + std::marker::Send + 'static> Listener<F> {
    pub(crate) fn start(self) -> () {
        thread::spawn(move || 'outer: loop {
            let config = (self.get_config)();
            let id = config.id.unwrap_or(get_cpu_serial().unwrap());
            let (client, mut connection) =
                Client::new(MqttOptions::new(&id, config.host, config.port), 64);

            client
                .subscribe("screens", rumqttc::QoS::AtLeastOnce)
                .unwrap();
            client
                .subscribe(format!("screens/{}", id), rumqttc::QoS::AtLeastOnce)
                .unwrap();

            let sender = self.sender.clone();

            for event in connection.iter() {
                println!("{:?}", event);

                if let Ok(rumqttc::Event::Incoming(Packet::Publish(Publish { payload, .. }))) =
                    event
                {
                    if let Ok(command) = serde_json::from_slice::<Command>(&payload) {
                        println!("{:?}", command);

                        if let Command::SetConfig(config) = command {
                            config.save().unwrap();
                            client.disconnect().unwrap();
                            continue 'outer;
                        }

                        sender.send(command).unwrap();
                    }
                }
            }
        });
    }
}
