use rumqttc::{Client, Event, LastWill, MqttOptions, Packet, QoS};
use serde_json;
use std::time::{Duration, SystemTime};
use rand::Rng;  
use reqwest;
use rand::rngs::StdRng;  
use rand::SeedableRng;  

use crate::json;

pub async fn post_mqtt_data(payload: json::MqttPayload) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let url = "http://localhost:7000/database/mqtt/data";

    let response = client
        .post(url)
        .json(&payload)
        .send()
        .await?;

    Ok(())
}

pub async fn start_mqtt_subscriber() {
    let mut mqttoptions = MqttOptions::new("mqtt_subscriber", "localhost", 1883);
    let will = LastWill::new("hello/world", "good-bye", QoS::AtMostOnce, false);
    mqttoptions
        .set_keep_alive(Duration::from_secs(60))
        .set_last_will(will);

    let (client, mut connection) = Client::new(mqttoptions, 10);

    client.subscribe("/mqtt_node_1/data", QoS::AtMostOnce).unwrap();
    client.subscribe("/mqtt_node_2/data", QoS::AtMostOnce).unwrap();
    
    println!("Subscribed to /mqtt_node_1/data, /mqtt_node_2/data");

    loop {
        match connection.eventloop.poll().await {
            Ok(Event::Incoming(Packet::Publish(publish))) => {
                println!(
                    "Received: Topic = {}, Payload = {:?}", publish.topic, publish.payload
                );

                match serde_json::from_slice::<json::MqttPayload>(&publish.payload) {
                    Ok(payload) => {
                        if let Err(e) = post_mqtt_data(payload).await {
                            eprintln!("Error sending MQTT data: {:?}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to deserialize MQTT payload: {:?}", e);
                    }
                }
            }
            Ok(_) => {}
            Err(e) => {
                println!("Error receiving message: {:?}", e);
                break;
            }
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    println!("MQTT subscriber disconnected or error occurred.");
}

pub async fn start_mqtt_publisher() {
    let mut mqttoptions = MqttOptions::new("mqtt_publisher", "localhost", 1883);
    let will = LastWill::new("hello/world", "good-bye", QoS::AtMostOnce, false);
    mqttoptions
        .set_keep_alive(Duration::from_secs(60))
        .set_last_will(will);

    let (client, mut connection) = Client::new(mqttoptions, 10);

    let mut rng = StdRng::from_entropy(); 

    loop {
        let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        let temperature = rng.gen_range(15.0..40.0);  
        let humidity = rng.gen_range(30..80);          
        let current = rng.gen_range(1.0..10.0);        

        let sensor_data = json::SensorData {
            temperature,
            humidity,
            current,
        };

        let payload = json::MqttPayload {
            id: String::from("mqtt_actix"),
            timestamp,
            data: sensor_data,
        };

        let json_data = serde_json::to_string(&payload).unwrap();
        
        client.publish("/mqtt_actix/data", QoS::AtMostOnce, false, json_data.clone()).unwrap();
        println!("Published: Topic = /mqtt_actix/data, Payload = {}", json_data); 

        match serde_json::from_str::<json::MqttPayload>(&json_data) {
            Ok(payload) => {
                if let Err(e) = post_mqtt_data(payload).await {
                    eprintln!("[publisher] Error post MQTT data: {:?}", e);
                }
            }
            Err(e) => {
                eprintln!("[publisher] Failed to deserialize MQTT payload: {:?}", e);
            }
        }

        tokio::time::sleep(Duration::from_secs(10)).await; 
    }

    println!("MQTT publisher disconnected or error occurred.");
}