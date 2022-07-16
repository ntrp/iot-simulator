use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};
use std::str::FromStr;
use std::sync::{Arc, RwLock};

use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd)]
pub enum GenerationResult {
    Int(i32),
    Float(f32),
    Str(String),
}

impl Default for GenerationResult {
    fn default() -> Self {
        GenerationResult::Str("MOCK".to_string())
    }
}

impl FromStr for GenerationResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(GenerationResult::Str(s.to_string()))
    }
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
            GenerationResult::Int(x) => write!(f, "{}", x),
            GenerationResult::Float(x) => write!(f, "{}", x),
            GenerationResult::Str(x) => write!(f, "{}", x),
        }
    }
}

impl Default for Box<dyn GeneratorPlugin> {
    fn default() -> Self {
        struct Anon();
        impl GeneratorPlugin for Anon {
            fn generate(&mut self, _: DateTime<Utc>) -> GenerationResult {
                GenerationResult::Int(1)
            }
        }
        Box::new(Anon())
    }
}

pub type GeneratorPointer = Arc<RwLock<dyn GeneratorPlugin + Send + Sync>>;

#[derive(Copy, Clone)]
pub struct GeneratorPluginDeclaration {
    pub rustc_version: &'static str,
    pub core_version: &'static str,
    pub generator_id: &'static str,
    pub instance_fn: unsafe fn(args: HashMap<String, String>) -> GeneratorPointer,
}

pub fn unwrap_arg<T: FromStr>(arg: &str, args: &HashMap<String, String>) -> T {
    match args
        .get(arg)
        .unwrap_or_else(|| panic!("No argument named {} available in the args map", arg))
        .parse::<T>()
    {
        Ok(val) => val,
        Err(_) => panic!("Failed to parse param '{}'", arg),
    }
}

#[macro_export]
macro_rules! export_plugin {
    ($generator_id:expr,$instance_fn:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static plugin_declaration: $crate::generator::GeneratorPluginDeclaration =
            $crate::generator::GeneratorPluginDeclaration {
                rustc_version: $crate::RUSTC_VERSION,
                core_version: $crate::CORE_VERSION,
                generator_id: $generator_id,
                instance_fn: $instance_fn,
            };
    };
}
