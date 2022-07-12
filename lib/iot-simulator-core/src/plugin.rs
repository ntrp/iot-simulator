use libloading::*;

use iot_simulator_api::generator::GeneratorPlugin;
use iot_simulator_api::output::OutputPlugin;

use crate::config::{GeneratorPluginConf, OutputPluginConf};

pub struct PluginFactory {}

pub struct PluginRegistry {}

pub fn load_generator_plugin(plugin: &GeneratorPluginConf) -> Box<dyn GeneratorPlugin> {
    unsafe {
        let lib = Library::new(&plugin.path).expect("Failed to load library");
        let factory_fn: Symbol<
            unsafe extern "C" fn(a: f32, b: f32, c: u32, d: usize) -> Box<dyn GeneratorPlugin>,
        > = lib.get(b"new_instance").expect("Failed to fetch symbol");
        factory_fn(10.0, 20.0, 2, 10)
    }
}

pub fn load_output_plugin(plugin: &OutputPluginConf) -> Box<dyn OutputPlugin> {
    unsafe {
        let lib = Library::new(&plugin.path).expect("Failed to load library");
        let factory_fn: Symbol<unsafe extern "C" fn() -> Box<dyn OutputPlugin>> =
            lib.get(b"new_instance").expect("Failed to fetch symbol");
        factory_fn()
    }
}
