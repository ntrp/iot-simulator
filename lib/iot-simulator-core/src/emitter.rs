use std::collections::HashMap;
use std::sync::Arc;

use async_stream::stream;
use chrono::{DateTime, Duration, Utc};
use tokio::time::sleep;
use tokio_stream::Stream;
use uuid::Uuid;

use iot_simulator_api::generator::GenerationResult;

use crate::parser::Sensor;

#[derive(Debug)]
#[allow(unused)]
pub struct SensorPayload {
    id: Uuid,
    device_path: String,
    name: String,
    metadata: HashMap<String, String>,
    timestamp: DateTime<Utc>,
    value: GenerationResult,
}

pub fn sensor_emitter(
    device_path: String,
    sensor: Arc<Sensor>,
    start_at: DateTime<Utc>,
    end_at: DateTime<Utc>,
) -> impl Stream<Item = SensorPayload> {
    stream! {
        let mut current = start_at;
        while current < Utc::now() || current < end_at {
            if current > Utc::now() {
                sleep((current - Utc::now()).to_std().unwrap()).await;
            }
            let payload = {
                let dpath = device_path.clone();
                let mut generator = sensor.value_generator.try_write()
                    .unwrap_or_else(|e| panic!("Failed to acquire lock on the sensor generator for sensor: {}/{} {:?}", &dpath, sensor.name, e));

                SensorPayload {
                    id: sensor.id.clone(),
                    device_path: dpath,
                    name: sensor.name.clone(),
                    metadata: sensor.metadata.clone(),
                    timestamp: current,
                    value: generator.generate()
                }
            };
            yield payload;
            current = current + Duration::milliseconds(sensor.sampling_rate);
        }
    }
}
