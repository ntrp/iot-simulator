use std::fs;

use crate::simulation::Simulation;

pub fn parse_simulation(file_path: String) -> Simulation {
    let config = fs::read_to_string(&file_path)
        .unwrap_or_else(|_| panic!("Failed to open file {}", &file_path));
    match ron::from_str(config.as_str()) {
        Ok(res) => res,
        Err(error) => panic!("{:?}", error),
    }
}
