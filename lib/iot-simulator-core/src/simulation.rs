use std::sync::Arc;

use chrono::{DateTime, Utc};
use tokio::select;
use tokio_stream::{Stream, StreamExt, StreamMap};
use uuid::Uuid;

use iot_simulator_api::channel::{ChannelPlugin, DefaultChannel, SimChannel};
use iot_simulator_api::output::SensorPayload;

use crate::emitter::sensor_emitter;
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
                    // FIXME: the generator is shared, we should register a new one for each replica
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
    let SimChannel { tx, mut rx } = DefaultChannel::init();

    println!("Starting simulation at: {}", Utc::now());

    let handle = tokio::spawn(async move {
        while let Ok(result) = rx.next().await {
            println!("Got {:?}", result)
        }
    });

    loop {
        select! {
            Some((_, payload)) = streams.next() => {
                tx.send(payload).await.unwrap_or_else(|e| panic!("Failed to send new payload: {}", e));
            },
            else => break
        }
    }
    handle.abort();
}
