use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use chrono::{DateTime, Utc};

use iot_simulator_api::export_plugin;
use iot_simulator_api::generator::{
    unwrap_arg, GenerationResult, GeneratorPlugin, GeneratorPointer,
};

unsafe fn new_instance(args: HashMap<String, String>) -> GeneratorPointer {
    let val: String = unwrap_arg("val", &args);
    ConstGenerator::new(GenerationResult::Str(val))
}

pub struct ConstGenerator {
    val: GenerationResult,
}

impl ConstGenerator {
    fn new(val: GenerationResult) -> Arc<RwLock<ConstGenerator>> {
        Arc::new(RwLock::new(ConstGenerator { val }))
    }
}

impl GeneratorPlugin for ConstGenerator {
    fn generate(&mut self, _: DateTime<Utc>) -> GenerationResult {
        self.val.clone()
    }
}

export_plugin!("const", new_instance);

#[cfg(test)]
#[path = "lib_tests.rs"]
mod lib_tests;
