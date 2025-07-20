use abi_stable::std_types::RHashMap;
use serde_json::{to_string, to_string_pretty};
use std::sync::{Arc, RwLock};

use iot_simulator_api::{
    export_output_plugin,
    output::{OutputPlugin, OutputPointer, SensorPayload},
};
use iot_simulator_core::utils::unwrap_arg;

#[derive(Debug)]
pub struct StdoutOutput {
    pretty: bool,
}

unsafe impl Sync for StdoutOutput {}
unsafe impl Send for StdoutOutput {}

impl StdoutOutput {
    fn new(pretty: bool) -> Arc<RwLock<StdoutOutput>> {
        Arc::new(RwLock::new(StdoutOutput { pretty: pretty }))
    }
}

impl OutputPlugin for StdoutOutput {
    fn send(&self, payload: SensorPayload) {
        if self.pretty {
            println!("{}", to_string_pretty(&payload).unwrap_or_else(|e| format!("Failed to serialize payload: {}", e)))
        } else {
            println!("OutputPlugin Got = {:?}", payload)
        };
    }
}

unsafe extern "C" fn new_instance(args: RHashMap<String, String>) -> OutputPointer {
    let pretty = unwrap_arg("pretty", &args);
    StdoutOutput::new(pretty)
}

export_output_plugin!("stdout", new_instance);

#[cfg(test)]
#[path = "lib_tests.rs"]
mod lib_tests;
