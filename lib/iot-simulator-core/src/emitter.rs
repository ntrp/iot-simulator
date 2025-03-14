use std::sync::Arc;

use async_stream::stream;
use chrono::{DateTime, Duration, Utc};
use tokio::time::sleep;
use tokio_stream::Stream;

use iot_simulator_api::output::SensorPayload;

use crate::parser::Sensor;

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
                    id: sensor.id,
                    device_path: dpath,
                    name: sensor.name.clone(),
                    metadata: sensor.metadata.clone(),
                    timestamp: current,
                    value: generator.generate()
                }
            };
            yield payload;
            current += Duration::milliseconds(sensor.sampling_rate);
        }
    }
}
