use iot_simulator_api::generator::*;

#[no_mangle]
pub fn new_instance() -> Box<dyn StatelessGeneratorPlugin<(), String>> {
    Box::new(FakerGenerator::new())
}

pub struct FakerGenerator {}

impl FakerGenerator {
    fn new() -> FakerGenerator {
        FakerGenerator {}
    }
}

impl GeneratorPlugin<(), String> for FakerGenerator {
    fn generate(&mut self, _: ()) -> String {
        "fake".to_string()
    }
}

impl StatelessGeneratorPlugin<(), String> for FakerGenerator {}
