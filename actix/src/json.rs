use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SensorData {
    pub temperature: f32,
    pub humidity: u32,
    pub current: f32,
}

#[derive(Serialize, Deserialize)]
pub struct MqttPayload {
    pub id: String,
    pub timestamp: f64,
    pub data: SensorData,
}