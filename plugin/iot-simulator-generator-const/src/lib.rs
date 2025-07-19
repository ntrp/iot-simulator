use abi_stable::std_types::RHashMap;
use iot_simulator_core::utils::unwrap_arg;
use std::sync::{Arc, RwLock};

use iot_simulator_api::export_plugin;
use iot_simulator_api::generator::{
    GenerationResult, GeneratorPlugin, GeneratorPointer,
};

unsafe extern "C" fn new_instance(args: RHashMap<String, String>) -> GeneratorPointer {
    let val: String = unwrap_arg("val", &args);
    ConstGenerator::new(GenerationResult::Str(val.parse().unwrap()))
}

#[repr(C)]
#[derive(Debug)]
pub struct ConstGenerator {
    val: GenerationResult,
}

impl ConstGenerator {
    fn new(val: GenerationResult) -> Arc<RwLock<ConstGenerator>> {
        Arc::new(RwLock::new(ConstGenerator { val }))
    }
}

impl GeneratorPlugin for ConstGenerator {
    fn generate(&mut self) -> GenerationResult {
        self.val.clone()
    }
}

export_plugin!("const", new_instance);

#[cfg(test)]
#[path = "lib_tests.rs"]
mod lib_tests;
