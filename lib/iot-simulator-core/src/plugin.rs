use abi_stable::std_types::RHashMap;
use iot_simulator_api::output::{
    OutputConfig, OutputPluginDeclaration, OutputPointer,
};
use libloading::*;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::mem::ManuallyDrop;
use std::sync::{Arc, RwLock};

use iot_simulator_api::generator::{GeneratorConfig, GeneratorPluginDeclaration, GeneratorPointer};

use crate::config::{GeneratorPluginConf, OutputPluginConf};

#[derive(Debug)]
pub struct GeneratorPluginRegistry {
    plugins: Vec<GeneratorPluginConf>,
    output_plugins: Vec<OutputPluginConf>,
    generators: RwLock<HashMap<String, GeneratorPointer>>,
    outputs: RwLock<HashMap<String, OutputPointer>>,
}

pub static GENERATOR_FACTORY_REGISTRY: OnceCell<GeneratorPluginRegistry> = OnceCell::new();

impl GeneratorPluginRegistry {
    pub fn instance() -> &'static GeneratorPluginRegistry {
        GENERATOR_FACTORY_REGISTRY
            .get()
            .expect("Registry not initialized")
    }
    pub fn init(plugins: Vec<GeneratorPluginConf>, output_plugins: Vec<OutputPluginConf>) {
        let registry: RwLock<HashMap<String, GeneratorPointer>> = RwLock::new(HashMap::new());
        let output_registry: RwLock<HashMap<String, OutputPointer>> = RwLock::new(HashMap::new());
        GENERATOR_FACTORY_REGISTRY
            .set(GeneratorPluginRegistry {
                plugins,
                output_plugins,
                generators: registry,
                outputs: output_registry,
            })
            .unwrap_or_else(|_| panic!("Failed to init registry"));
    }

    pub fn get_outputs(&self) -> Vec<OutputPointer> {
        self.outputs.read().unwrap().values().cloned().collect()
    }

    pub fn register(generator_conf: &mut GeneratorConfig) -> GeneratorPointer {
        let registry = GeneratorPluginRegistry::instance();
        let plugins = &registry.plugins;
        let conf = plugins
            .iter()
            .find(|plugin| plugin.id == generator_conf.generator_id)
            .unwrap_or_else(|| {
                panic!(
                    "No plugin configured for generator_id: {}",
                    generator_conf.generator_id
                )
            });
        let mut generator_map = registry
            .generators
            .write()
            .expect("Cannot acquire write lock on the generator registry");
        let generator = generator_map
            .entry(generator_conf.instance_id.clone())
            .or_insert_with(|| init_generator(conf, generator_conf.params.clone()));
        generator.clone()
    }
    pub fn register_output(output_conf: &mut OutputConfig) -> OutputPointer {
        let registry = GeneratorPluginRegistry::instance();
        let plugins = &registry.output_plugins;
        let conf = plugins
            .iter()
            .find(|plugin| plugin.id == output_conf.output_id)
            .unwrap_or_else(|| {
                panic!(
                    "No plugin configured for output_id: {}",
                    output_conf.output_id
                )
            });
        let mut output_map = registry
            .outputs
            .write()
            .expect("Cannot acquire write lock on the output registry");
        let output = output_map
            .entry(&output_conf.instance_id)
            .or_insert_with(|| init_output(conf, output_conf.params.clone()));
        output.clone()
    }
}

pub fn init_generator(
    plugin: &GeneratorPluginConf,
    args: RHashMap<String, String>,
) -> GeneratorPointer {
    unsafe {
        // Due to the fact we don't care about unloading the libs at runtime we can leak the references
        let lib = ManuallyDrop::new(Arc::new(
            Library::new(&plugin.path).expect("Failed to load library"),
        ));
        let declaration = lib
            .get::<*mut GeneratorPluginDeclaration>(b"PLUGIN_DECLARATION\0")
            .unwrap_or_else(|_| panic!("Missing plugin declaration for {}", plugin.path))
            .read();
        (declaration.instance_fn)(args)
    }
}

pub fn init_output(plugin: &OutputPluginConf, args: RHashMap<String, String>) -> OutputPointer {
    unsafe {
        // Due to the fact we don't care about unloading the libs at runtime we can leak the references
        let lib = ManuallyDrop::new(Arc::new(
            Library::new(&plugin.path).expect("Failed to load library"),
        ));
        let declaration = lib
            .get::<*mut OutputPluginDeclaration>(b"PLUGIN_DECLARATION\0")
            .unwrap_or_else(|_| panic!("Missing plugin declaration for {}", plugin.path))
            .read();
        (declaration.instance_fn)(args)
    }
}
