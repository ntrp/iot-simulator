use abi_stable::std_types::RString;
use abi_stable::{sabi_trait, std_types::RHashMap, StableAbi};
use std::fmt::{Debug, Display, Error, Formatter};
use std::str::FromStr;
use std::sync::{Arc, RwLock};

use serde::Deserialize;
use uuid::Uuid;

// TODO: make the interface fully abi stable via the abi_stable package

#[repr(C)]
#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd, StableAbi)]
pub enum GenerationResult {
    Int(i32),
    Float(f32),
    Str(RString),
}

impl Default for GenerationResult {
    fn default() -> Self {
        GenerationResult::Str(RString::from("MOCK"))
    }
}

impl FromStr for GenerationResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(GenerationResult::Str(RString::from(s.to_string())))
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
    #[serde(default = "RHashMap::new")]
    pub params: RHashMap<String, String>,
}

fn default_instance_id() -> String {
    Uuid::new_v4().to_string()
}

#[sabi_trait]
pub trait GeneratorPlugin: Send + Sync + Debug {
    fn generate(&mut self) -> GenerationResult;
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

pub fn get_mock_generator() -> Arc<RwLock<dyn GeneratorPlugin>> {
    #[derive(Debug)]
    struct Anon();
    impl GeneratorPlugin for Anon {
        fn generate(&mut self) -> GenerationResult {
            GenerationResult::Int(1)
        }
    }
    Arc::new(RwLock::new(Anon()))
}

pub type GeneratorPointer = Arc<RwLock<dyn GeneratorPlugin>>;

#[derive(Copy, Clone)]
pub struct GeneratorPluginDeclaration {
    pub rustc_version: &'static str,
    pub core_version: &'static str,
    pub generator_id: &'static str,
    pub instance_fn: unsafe extern "C" fn(args: RHashMap<String, String>) -> GeneratorPointer,
}

pub fn unwrap_arg<T: FromStr>(arg: &str, args: &RHashMap<String, String>) -> T {
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
