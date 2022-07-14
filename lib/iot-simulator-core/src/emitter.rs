use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

use async_stream::stream;
use chrono::{DateTime, Duration, Utc};
use futures_util::Stream;
use tokio::sync::Notify;
use uuid::Uuid;

use iot_simulator_api::generator::GenerationResult;

use crate::simulation::Sensor;

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
    sensor: &mut Sensor,
    start_at: DateTime<Utc>,
    end_at: DateTime<Utc>,
) -> impl Stream<Item=SensorPayload> + '_ {
    stream! {
        let mut current = start_at;
        while current < Utc::now() || current < end_at {
            if current > Utc::now() {
                delay(current - Utc::now()).await;
            }
            yield SensorPayload {
                id: sensor.id.clone(),
                device_path: device_path.clone(),
                name: sensor.name.clone(),
                metadata: sensor.metadata.clone(),
                timestamp: current,
                value: sensor.value_generator.generate(current)
            };
            current = current + Duration::milliseconds(sensor.sampling_rate);
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
