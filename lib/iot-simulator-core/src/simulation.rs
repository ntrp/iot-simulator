use std::io::repeat;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use tokio::select;
use tokio_stream::{Stream, StreamExt, StreamMap};
use uuid::Uuid;

use crate::emitter::{sensor_emitter, SensorPayload};
use crate::parser::{Device, Simulation};

fn generate_emitters(
    path: String,
    start_at: DateTime<Utc>,
    end_at: DateTime<Utc>,
    devices: Vec<Device>,
) -> Vec<(String, impl Stream<Item = SensorPayload>)> {
    let mut emitters = vec![];
    for device in devices {
        let full_path = format!("{}/{}", path, device.name);
        for sensor in device.sensors {
            for i in 0..sensor.replicate {
                let mut dup = sensor.clone();
                if dup.replicate > 1 {
                    dup.id = Uuid::new_v4();
                    dup.name += &*i.to_string();
                }
                emitters.push((
                    format!("{}/{}", full_path, dup.id),
                    Box::pin(sensor_emitter(
                        full_path.clone(),
                        Arc::from(dup),
                        start_at,
                        end_at,
                    )),
                ));
            }
        }
        if !device.devices.is_empty() {
            emitters.append(&mut generate_emitters(
                full_path.clone(),
                start_at,
                end_at,
                device.devices,
            ));
        }
    }
    emitters
}

pub async fn run(simulation: Simulation) {
    let start_at = simulation.start_at;
    let end_at = simulation.end_at;
    let emitters = generate_emitters("".into(), start_at, end_at, simulation.devices);

    let mut streams = StreamMap::from_iter(emitters);

    println!("{}", streams.len());

    println!("Starting simulation at: {}", Utc::now());
    loop {
        select! {
            Some((channel, payload)) = streams.next() => {
                println!("Got {:?} from {}", payload, channel)
            },
            else => break
        }
    }
}
