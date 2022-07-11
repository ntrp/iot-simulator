use clap::Parser;

use iot_simulator_core::parser::parse_simulation;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Simulation configuration file
    #[clap(short, long, value_parser)]
    config_file: String,
}

fn main() {
    let args = Args::parse();
    let simulation = parse_simulation(args.config_file);
    println!("{:?}", simulation)
}
