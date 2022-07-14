use std::collections::HashMap;

use libloading::*;
use once_cell::sync::OnceCell;

use iot_simulator_api::generator::GeneratorPlugin;
use iot_simulator_api::output::OutputPlugin;

use crate::config::{GeneratorPluginConf, OutputPluginConf};

type GeneratorPluginFactoryFn = fn() -> Box<dyn GeneratorPlugin>;

#[derive(Debug)]
pub struct GeneratorPluginFactoryRegistry {
    registry: HashMap<String, GeneratorPluginFactoryFn>,
}

pub static GENERATOR_FACTORY_REGISTRY: OnceCell<GeneratorPluginFactoryRegistry> = OnceCell::new();

impl GeneratorPluginFactoryRegistry {
    pub fn instance() -> &'static GeneratorPluginFactoryRegistry {
        GENERATOR_FACTORY_REGISTRY
            .get()
            .expect("Registry not initialized")
    }
    pub fn get(generator_id: &String) -> Option<&'static GeneratorPluginFactoryFn> {
        GeneratorPluginFactoryRegistry::instance().registry.get(generator_id)
    }
    pub fn init(plugins: Vec<GeneratorPluginConf>) {
        let mut registry: HashMap<String, GeneratorPluginFactoryFn> = HashMap::new();
        for plugin in plugins {
            let factory: Box<dyn GeneratorPlugin> = load_generator_plugin(&plugin);
            registry.insert(plugin.id, Default::default);
        }
        GENERATOR_FACTORY_REGISTRY
            .set(GeneratorPluginFactoryRegistry { registry })
            .unwrap()
    }
}

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
