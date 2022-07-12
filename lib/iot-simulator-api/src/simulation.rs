use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sensor {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    pub name: String,
    #[serde(default = "HashMap::new")]
    pub metadata: HashMap<String, String>,
    pub sampling_rate: i64,
    pub value_generator: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    #[serde(default = "Uuid::new_v4")]
    id: Uuid,
    name: String,
    #[serde(default = "HashMap::new")]
    metadata: HashMap<String, String>,
    #[serde(default = "Vec::new")]
    sensors: Vec<Sensor>,
    #[serde(default = "Vec::new")]
    devices: Vec<Device>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Simulation {
    name: String,
    #[serde(default = "String::new")]
    description: String,
    #[serde(default = "Utc::now")]
    start_at: DateTime<Utc>,
    end_at: Option<DateTime<Utc>>,
    #[serde(default = "Vec::new")]
    devices: Vec<Device>,
    #[serde(default = "Vec::new")]
    output_plugins: Vec<String>,
}
