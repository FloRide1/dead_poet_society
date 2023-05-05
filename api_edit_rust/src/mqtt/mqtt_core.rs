use rocket::futures::lock::Mutex;

use rumqttc::v5::{mqttbytes::QoS, AsyncClient, MqttOptions};

// Used as a singleton
lazy_static! {
    pub static ref CLIENT: Mutex<Option<AsyncClient>> = Mutex::new(None);
}

pub async fn mqtt_login(host: &str, port: u16) {
    let mut mqttoptions = MqttOptions::new("api_edit_rust", host, port);
    mqttoptions.set_keep_alive(std::time::Duration::from_secs(5));
    let (client, eventloop) = AsyncClient::new(mqttoptions, 100);
    *CLIENT.lock().await = Some(client);

    rocket::tokio::spawn(async { eventloop_handler(eventloop).await });
}

async fn eventloop_handler(mut eventloop: rumqttc::v5::EventLoop) {
    loop {
        while let Ok(_notification) = eventloop.poll().await {
            // println!("Notification = {:?}", notification);
        }
    }
}

pub async fn mqtt_publish(channel: &str, obj: impl serde::Serialize) {
    if let Some(client) = &*CLIENT.lock().await {
        client
            .publish(
                channel,
                QoS::AtLeastOnce,
                false,
                bincode::serialize(&obj).expect("Obj cannot be serialize into byte"),
            )
            .await
            .expect("Client publish failed");
    }
}

pub async fn mqtt_publish_json(channel: &str, obj: impl serde::Serialize) {
    if let Some(client) = &*CLIENT.lock().await {
        client
            .publish(
                channel,
                QoS::AtLeastOnce,
                false,
                bincode::serialize(&serde_json::to_string(&obj).expect("Obj cannot be serialize into json")).expect("Obj Json cannot be serialize into byte"),
            )
            .await
            .expect("Client publish failed");
    }
}
