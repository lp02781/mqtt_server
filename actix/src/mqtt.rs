use rumqttc::{Client, Event, LastWill, MqttOptions, Packet, QoS};
use serde::{Serialize};
use serde_json;
use std::time::{Duration, SystemTime};
use rand::Rng;  
use rand::rngs::StdRng;  
use rand::SeedableRng;  

pub async fn start_mqtt_subscriber() {
    let mut mqttoptions = MqttOptions::new("mqtt_subscriber", "localhost", 1883);
    let will = LastWill::new("hello/world", "good-bye", QoS::AtMostOnce, false);
    mqttoptions
        .set_keep_alive(Duration::from_secs(5))
        .set_last_will(will);

    let (client, mut connection) = Client::new(mqttoptions, 10);

    client.subscribe("/node_sim_1/data", QoS::AtMostOnce).unwrap();
    client.subscribe("/node_sim_2/data", QoS::AtMostOnce).unwrap();
    client.subscribe("/node_sim_3/data", QoS::AtMostOnce).unwrap();
    client.subscribe("/node_sim_4/data", QoS::AtMostOnce).unwrap();
    println!("Subscribed to /node_sim_1/data, /node_sim_2/data, /node_sim_3/data, and /node_sim_4/data");

    loop {
        match connection.eventloop.poll().await {
            Ok(Event::Incoming(Packet::Publish(publish))) => {
                println!("Received: Topic = {}, Payload = {:?}", publish.topic, publish.payload);
            }
            Ok(_) => {}
            Err(e) => {
                println!("Error receiving message: {:?}", e);
                break; 
            }
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    println!("MQTT client disconnected or error occurred.");
}

pub async fn start_mqtt_publisher() {
    #[derive(Serialize)]
    struct SensorData {
        temperature: f32,
        humidity: u32,
        current: f32,
    }

    #[derive(Serialize)]
    struct MqttPayload {
        id: String,
        timestamp: f64,
        data: SensorData,
    }
    
    let mut mqttoptions = MqttOptions::new("mqtt_publisher", "localhost", 1883);
    let will = LastWill::new("hello/world", "good-bye", QoS::AtMostOnce, false);
    mqttoptions
        .set_keep_alive(Duration::from_secs(5))
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

        let sensor_data = SensorData {
            temperature,
            humidity,
            current,
        };

        let payload = MqttPayload {
            id: String::from("mqtt_actix"),
            timestamp,
            data: sensor_data,
        };

        let json_data = serde_json::to_string(&payload).unwrap();
        
        client.publish("/mqtt_actix/data", QoS::AtMostOnce, false, json_data.clone()).unwrap();
        println!("Published: Topic = /mqtt_actix/data, Payload = {}", json_data); 

        tokio::time::sleep(Duration::from_secs(10)).await; 
    }

    println!("MQTT client disconnected or error occurred.");
}
