use clap::Parser;

use iot_simulator_core::config::load_settings;
use iot_simulator_core::parser::parse_simulation;
use iot_simulator_core::plugin::GeneratorPluginRegistry;

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
async fn main() {
    let args = Args::parse();
    let settings = load_settings(args.config_file);
    GeneratorPluginRegistry::init(settings.generator_plugins);
    let simulation = parse_simulation(args.simulation_file);
    println!("{:?}", GeneratorPluginRegistry::instance());
    iot_simulator_core::simulation::run(simulation).await;
}
