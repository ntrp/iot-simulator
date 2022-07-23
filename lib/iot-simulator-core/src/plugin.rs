use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::mem::ManuallyDrop;
use std::sync::{Arc, RwLock};

use abi_stable::reexports::SelfOps;
use abi_stable::std_types::RHashMap;
use libloading::*;
use once_cell::sync::OnceCell;
use uuid::Uuid;

use iot_simulator_api::generator::{
    GeneratorConfig, GeneratorPluginDeclaration, GeneratorPointer, GeneratorType,
};

use crate::config::GeneratorPluginConf;

#[derive(Debug)]
pub struct GeneratorPluginRegistry {
    plugins: Vec<GeneratorPluginConf>,
    generators: RwLock<HashMap<String, GeneratorPointer>>,
}

pub static GENERATOR_FACTORY_REGISTRY: OnceCell<GeneratorPluginRegistry> = OnceCell::new();

impl GeneratorPluginRegistry {
    pub fn instance() -> &'static GeneratorPluginRegistry {
        GENERATOR_FACTORY_REGISTRY
            .get()
            .expect("Registry not initialized")
    }
    pub fn init(plugins: Vec<GeneratorPluginConf>) {
        let registry: RwLock<HashMap<String, GeneratorPointer>> = RwLock::new(HashMap::new());
        GENERATOR_FACTORY_REGISTRY
            .set(GeneratorPluginRegistry {
                plugins,
                generators: registry,
            })
            .unwrap_or_else(|_| panic!("Failed to init registry"));
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
            .get::<*mut GeneratorPluginDeclaration>(b"plugin_declaration\0")
            .unwrap_or_else(|_| panic!("Missing plugin declaration for {}", plugin.path))
            .read();
        (declaration.instance_fn)(args)
    }
}
