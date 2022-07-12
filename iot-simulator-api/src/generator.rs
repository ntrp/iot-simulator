use std::fmt;
use std::fmt::Formatter;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum GenerationResult {
    ResultI32(i32),
    ResultF32(f32),
    ResultString(String),
}

impl fmt::Display for GenerationResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GenerationResult::ResultI32(x) => write!(f, "{}", x),
            GenerationResult::ResultF32(x) => write!(f, "{}", x),
            GenerationResult::ResultString(x) => write!(f, "{}", x)
        }
    }
}

#[derive(Debug, Deserialize)]
pub enum GeneratorType {
    Stateless,
    Stateful,
}

pub trait GeneratorPlugin {
    fn generate(&mut self, time: DateTime<Utc>) -> GenerationResult;
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
