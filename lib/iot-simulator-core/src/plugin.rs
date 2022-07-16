use std::collections::HashMap;
use std::rc::Rc;
use std::sync::RwLock;

use libloading::*;
use once_cell::sync::OnceCell;

use iot_simulator_api::generator::{GeneratorConfig, GeneratorPluginDeclaration, GeneratorPointer};

use crate::config::GeneratorPluginConf;

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
    pub fn register(generator_conf: &GeneratorConfig) -> GeneratorPointer {
        let plugins = &GeneratorPluginRegistry::instance().plugins;
        let conf = plugins
            .iter()
            .find(|plugin| plugin.id == generator_conf.generator_id)
            .unwrap_or_else(|| {
                panic!(
                    "No plugin configured for generator_id: {}",
                    generator_conf.generator_id
                )
            });
        let generator = init_generator(conf, generator_conf.params.clone());
        GeneratorPluginRegistry::instance()
            .generators
            .write()
            .expect("Unable to acquire a lock on the generator registry")
            .insert(generator_conf.generator_id.clone(), generator.clone());
        generator
    }
}

pub fn init_generator(
    plugin: &GeneratorPluginConf,
    args: HashMap<String, String>,
) -> GeneratorPointer {
    unsafe {
        let lib = Rc::new(Library::new(&plugin.path).expect("Failed to load library"));
        let declaration = lib
            .get::<*mut GeneratorPluginDeclaration>(b"plugin_declaration\0")
            .unwrap_or_else(|_| panic!("Missing plugin declaration for {}", plugin.path))
            .read();
        (declaration.instance_fn)(args)
    }
}
