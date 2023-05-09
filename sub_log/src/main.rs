use dotenvy::dotenv;
use rumqttc::v5::{
    mqttbytes::{v5::Packet, QoS},
    Client, ClientError, Event, MqttOptions,
};
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
use std::time::Duration;

#[allow(unused_must_use)]

fn main() {
    println!("Hello, sub_log!");

    dotenv().ok();
    pretty_env_logger::init();
    info!("env has been loaded");

    let mqtt_host = std::env::var("MQTT_HOST").expect("MQTT_HOST is not set");
    info!("MQTT_HOST is set to {}", &mqtt_host);

    let mqtt_port = std::env::var("MQTT_PORT")
        .expect("MQTT_PORT is not set")
        .parse::<u16>()
        .expect("MQTT_PORT is not a number");
    info!("MQTT_PORT is set to {}", mqtt_port);

    let mut mqttoptions = MqttOptions::new("sub_log", &mqtt_host, mqtt_port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut connection) = Client::new(mqttoptions, 30);
    info!(
        "New MQTT Client has been created using {}:{}",
        &mqtt_host, mqtt_port
    );

    // Writers
    subscribe(&client, "new_writer_json");
    subscribe(&client, "new_writer_confirmed_json");
    subscribe(&client, "edit_writer_json");
    subscribe(&client, "edit_writer_confirmed_json");
    subscribe(&client, "delete_writer_json");
    subscribe(&client, "delete_writer_confirmed_json");

    // Circles
    subscribe(&client, "new_circle_json");
    subscribe(&client, "new_circle_confirmed_json");
    subscribe(&client, "edit_circle_json");
    subscribe(&client, "edit_circle_confirmed_json");
    subscribe(&client, "delete_circle_json");
    subscribe(&client, "delete_circle_confirmed_json");

    // Letters
    subscribe(&client, "new_letter_json");
    subscribe(&client, "new_letter_confirmed_json");
    subscribe(&client, "delete_letter_json");
    subscribe(&client, "delete_letter_confirmed_json");

    for notification in connection.iter() {
        if let Err(err) = notification {
            error!("Error = {:?}", err);
            continue;
        }

        let notification = notification.unwrap();
        if let Event::Incoming(Packet::Publish(p)) = notification {
            let topic = std::str::from_utf8(&p.topic).unwrap();
            let payload: String = bincode::deserialize(&p.payload).unwrap();

            info!("{} = {:?}", topic, payload);
        }
    }
}

fn subscribe(client: &Client, topic: &str) -> Result<(), ClientError> {
    match client.subscribe(topic, QoS::AtMostOnce) {
        Ok(_) => {
            info!("Succesfully subscribe to topic: {}", topic);

            Ok(())
        }

        Err(err) => {
            warn!("Error cannot subscribe to topic: {}", topic);
            Err(err)
        }
    }
}
