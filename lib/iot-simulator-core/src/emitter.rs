use async_stream::stream;
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

use chrono::{DateTime, Duration, Utc};
use futures_util::Stream;
use tokio::sync::Notify;
use uuid::Uuid;

use iot_simulator_api::generator::GenerationResult;
use iot_simulator_api::simulation::Sensor;

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
    sensor: Sensor,
    start_at: DateTime<Utc>,
    end_at: DateTime<Utc>,
) -> impl Stream<Item = SensorPayload> {
    stream! {
        let mut current = start_at;
        let Sensor {
            id,
            name,
            metadata,
            sampling_rate,
            value_generator: _,
        } = sensor.clone();
        while current < Utc::now() || current < end_at {
            if current > Utc::now() {
                delay(current - Utc::now()).await;
            }
            yield SensorPayload {
                id,
                device_path: device_path.clone(),
                name: name.clone(),
                metadata: metadata.clone(),
                timestamp: current,
                value: GenerationResult::ResultF32(1.0)
            };
            current = current + Duration::milliseconds(sampling_rate);
        }
    }
}

async fn delay(dur: Duration) {
    let when = Utc::now() + dur;
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();
    thread::spawn(move || {
        let now = Utc::now();
        if now < when {
            thread::sleep((when - now).to_std().expect("Duration conversion failed"));
        }
        notify2.notify_one();
    });
    notify.notified().await;
}

#[cfg(test)]
#[path = "emitter_test.rs"]
mod emitter_test;
