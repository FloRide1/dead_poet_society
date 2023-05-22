// use rocket::futures::lock::Mutex;

use std::sync::Mutex;

use rumqttc::v5::{mqttbytes::QoS, Client, MqttOptions};

// Used as a singleton
lazy_static! {
    pub static ref CLIENT: Mutex<Option<Client>> = Mutex::new(None);
}

pub fn mqtt_login(host: &str, port: u16) {
    let mut mqttoptions = MqttOptions::new("api_edit_rust", host, port);
    mqttoptions.set_keep_alive(std::time::Duration::from_secs(5));
    let (client, connection) = Client::new(mqttoptions, 100);
    *CLIENT.lock().unwrap() = Some(client);

    std::thread::spawn(|| connection_handler(connection));
}

fn connection_handler(mut connection: rumqttc::v5::Connection) {
    loop {
        for (_i, _notification) in connection.iter().enumerate() {
            // println!("Notification = {:?}", notification);
        }
    }
}

pub fn mqtt_publish(channel: &str, obj: impl serde::Serialize) {
    if let Some(client) = &*CLIENT.lock().unwrap() {
        client
            .publish(
                channel,
                QoS::AtLeastOnce,
                false,
                bincode::serialize(&obj).expect("Obj cannot be serialize into byte"),
            )
            .expect("Client publish failed");
    }
}

pub fn mqtt_publish_json(channel: &str, obj: impl serde::Serialize) {
    if let Some(client) = &*CLIENT.lock().unwrap() {
        client
            .publish(
                channel,
                QoS::AtLeastOnce,
                false,
                bincode::serialize(
                    &serde_json::to_string(&obj).expect("Obj cannot be serialize into json"),
                )
                .expect("Obj Json cannot be serialize into byte"),
            )
            .expect("Client publish failed");
    }
}
