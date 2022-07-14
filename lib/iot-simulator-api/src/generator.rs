use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub enum GenerationResult {
    ResultI32(i32),
    ResultF32(f32),
    ResultString(String),
}

#[derive(Debug, Deserialize)]
pub enum GeneratorType {
    Stateless,
    Stateful,
}

#[derive(Debug, Deserialize)]
pub struct GeneratorConfig {
    pub generator_id: String,
    #[serde(default = "default_instance_id")]
    pub instance_id: String,
    #[serde(default = "HashMap::new")]
    pub params: HashMap<String, String>,
}

fn default_instance_id() -> String {
    Uuid::new_v4().to_string()
}

pub trait GeneratorPlugin {
    fn generate(&mut self, time: DateTime<Utc>) -> GenerationResult;
}

impl Display for GenerationResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            GenerationResult::ResultI32(x) => write!(f, "{}", x),
            GenerationResult::ResultF32(x) => write!(f, "{}", x),
            GenerationResult::ResultString(x) => write!(f, "{}", x)
        }
    }
}

impl Default for Box<dyn GeneratorPlugin> {
    fn default() -> Self {
        struct Anon();
        impl GeneratorPlugin for Anon {
            fn generate(&mut self, _: DateTime<Utc>) -> GenerationResult {
                GenerationResult::ResultI32(1)
            }
        }
        Box::new(Anon())
    }
}


// FIXME: rework plugin loading system
// https://adventures.michaelfbryan.com/posts/plugins-in-rust/
//#[derive(Copy, Clone)]
//pub struct GeneratorPluginDeclaration {
//    pub rustc_version: &'static str,
//    pub core_version: &'static str,
//    pub register: unsafe extern "C" fn(&mut dyn GeneratorPluginRegistry),
//}
//
//pub trait GeneratorPluginRegistry {
//    fn register_function(&mut self, name: &str, plugin: Box<dyn GeneratorPlugin<_, O>>);
//}
//
//#[macro_export]
//macro_rules! export_plugin {
//    ($register:expr) => {
//        #[doc(hidden)]
//        #[no_mangle]
//        pub static plugin_declaration: $crate::PluginDeclaration =
//            $crate::PluginDeclaration {
//                rustc_version: $crate::RUSTC_VERSION,
//                core_version: $crate::CORE_VERSION,
//                register: $register,
//            };
//    };
//}
