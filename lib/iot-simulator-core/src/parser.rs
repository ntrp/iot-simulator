use std::fs;

use iot_simulator_api::simulation::*;

pub fn parse_simulation(file_path: String) -> Simulation {
    let config = fs::read_to_string(&file_path)
        .expect(format!("Failed to open file {}", &file_path).as_str());
    match ron::from_str(config.as_str()) {
        Ok(res) => res,
        Err(error) => panic!("{:?}", error),
    }
}
