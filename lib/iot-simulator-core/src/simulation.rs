use std::collections::HashMap;

use chrono::{DateTime, Utc};
use derivative::Derivative;
use serde::{Deserialize, Deserializer};
use uuid::Uuid;

use iot_simulator_api::generator::{GeneratorConfig, GeneratorPlugin};

use crate::plugin::GeneratorPluginFactoryRegistry;

#[derive(Derivative, Deserialize)]
#[derivative(Debug)]
pub struct Sensor {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    pub name: String,
    #[serde(default = "HashMap::new")]
    pub metadata: HashMap<String, String>,
    pub sampling_rate: i64,
    #[serde(deserialize_with = "to_generator")]
    #[derivative(Debug = "ignore")]
    pub value_generator: Box<dyn GeneratorPlugin>,
}

pub fn to_generator<'de, D>(deserializer: D) -> Result<Box<dyn GeneratorPlugin>, D::Error>
    where
        D: Deserializer<'de>,
{
    let config = GeneratorConfig::deserialize(deserializer)?;
    match GeneratorPluginFactoryRegistry::get(&config.generator_id) {
        Some(factory) => Ok(factory()),
        None => panic!(
            "Cannot find factory for generator id {}",
            config.generator_id
        ),
    }
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Device {
    #[serde(default = "Uuid::new_v4")]
    id: Uuid,
    name: String,
    #[serde(default = "HashMap::new")]
    metadata: HashMap<String, String>,
    #[serde(default = "Vec::new")]
    pub sensors: Vec<Sensor>,
    #[serde(default = "Vec::new")]
    devices: Vec<Device>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Simulation {
    name: String,
    #[serde(default = "String::new")]
    description: String,
    #[serde(default = "Utc::now")]
    start_at: DateTime<Utc>,
    end_at: Option<DateTime<Utc>>,
    #[serde(default = "Vec::new")]
    pub devices: Vec<Device>,
    #[serde(default = "Vec::new")]
    output_plugins: Vec<String>,
}
