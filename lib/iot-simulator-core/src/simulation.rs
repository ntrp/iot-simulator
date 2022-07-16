use std::sync::Arc;

use chrono::Utc;
use futures_util::future::join_all;
use futures_util::pin_mut;
use tokio_stream::StreamExt;

use crate::emitter::sensor_emitter;
use crate::parser::{Device, Sensor, Simulation};

struct SensorIter {
    stack: Vec<Arc<Sensor>>,
    devices: Vec<Device>,
}

impl SensorIter {
    fn new(simulation: Simulation) -> Self {
        SensorIter {
            stack: vec![],
            devices: simulation.devices,
        }
    }
}

impl Iterator for SensorIter {
    type Item = Arc<Sensor>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(device) = self.devices.pop() {
            for device in device.devices {
                self.devices.push(device);
            }
            for sensor in device.sensors {
                self.stack.push(Arc::from(sensor));
            }
        }
        if let Some(sensor) = self.stack.pop() {
            return Some(sensor);
        }
        None
    }
}

pub async fn run(simulation: Simulation) {
    let start_at = simulation.start_at;
    let end_at = simulation.end_at;

    let iter = SensorIter::new(simulation);

    let emitters = iter.map(|sensor| sensor_emitter("".to_string(), sensor, start_at, end_at));

    let mut handles = Vec::new();
    for emitter in emitters {
        let handle = tokio::spawn(async {
            pin_mut!(emitter);
            while let Some(value) = emitter.next().await {
                println!("got {:?}", value);
            }
        });
        handles.push(handle)
    }
    println!("Starting simulation at: {}", Utc::now());
    join_all(handles).await;
}
