use chrono::Utc;
use clap::Parser;

use iot_simulator_core::config::load_settings;
use iot_simulator_core::parser::parse_simulation;
use iot_simulator_core::plugin::load_generator_plugin;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    config_file: Option<String>,
    /// Simulation configuration file
    #[clap(short, long, value_parser)]
    simulation_file: String,
}

fn main() {
    let args = Args::parse();
    let settings = load_settings(args.config_file);
    let mut plugin = load_generator_plugin(&settings.generator_plugins[0]);
    let _ = parse_simulation(args.simulation_file);
    for _ in 1..15 {
        println!("{}", plugin.generate(Utc::now()));
    }
}
