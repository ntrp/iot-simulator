use std::pin::Pin;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use tokio::select;
use tokio_stream::{Stream, StreamExt, StreamMap};
use uuid::Uuid;

use iot_simulator_api::channel::{ChannelPlugin, InMemoryChannel};
use iot_simulator_api::output::SensorPayload;

use crate::emitter::sensor_emitter;
use crate::parser::{Device, Simulation};
use crate::plugin::GeneratorPluginRegistry;

fn generate_emitters(
    path: String,
    start_at: DateTime<Utc>,
    end_at: DateTime<Utc>,
    devices: Vec<Device>,
) -> Vec<(String, Pin<Box<dyn Stream<Item = SensorPayload>>>)> {
    let mut emitters = vec![];
    for device in devices {
        let full_path = format!("{}/{}", path, device.name);
        for sensor in device.sensors {
            for i in 0..sensor.replicate {
                let mut dup = sensor.clone();
                if dup.replicate > 1 {
                    dup.id = Uuid::new_v4();
                    dup.name += &*i.to_string();
                    // FIXME: the generator is shared, we should register a new one for each replica
                }
                emitters.push((
                    format!("{}/{}", full_path, dup.id),
                    sensor_emitter(full_path.clone(), Arc::from(dup), start_at, end_at),
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
    let mut stream = StreamMap::from_iter(emitters);

    let mut channel = InMemoryChannel::new();

    let registry = GeneratorPluginRegistry::instance();
    let outputs = registry.get_outputs();
    for output in outputs {
        channel.subscribe(output.clone());
    }

    println!("Starting simulation at: {}", Utc::now());

    loop {
        select! {
            Some((_, payload)) = stream.next() => {
                channel.send(payload).unwrap_or_else(|e| panic!("Failed to send new payload: {}", e));
            },
            else => break
        }
    }
}
