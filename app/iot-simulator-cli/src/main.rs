use std::error::Error;

use chrono::{Duration, Utc};
use clap::Parser;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

use iot_simulator_core::config::load_settings;
use iot_simulator_core::emitter::sensor_emitter;
use iot_simulator_core::parser::parse_simulation;
use iot_simulator_core::plugin::GeneratorPluginFactoryRegistry;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    config_file: Option<String>,
    /// Simulation configuration file
    #[clap(short, long, value_parser)]
    simulation_file: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let settings = load_settings(args.config_file);
    GeneratorPluginFactoryRegistry::init(settings.generator_plugins);
    let mut simulation = parse_simulation(args.simulation_file);
    let sensor = &mut simulation.devices[0].sensors[0];
    let emitter = sensor_emitter(
        "".to_string(),
        sensor,
        Utc::now() - Duration::seconds(5),
        Utc::now(),
    );

    pin_mut!(emitter);

    while let Some(value) = emitter.next().await {
        println!("got {:?}", value);
    }
    Ok(())
}
