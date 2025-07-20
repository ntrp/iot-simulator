use abi_stable::std_types::RHashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::Arc;
use std::{collections::HashMap, sync::RwLock};
use uuid::Uuid;

use crate::generator::GenerationResult;

#[derive(Debug, Clone, Serialize)]
#[allow(unused)]
pub struct SensorPayload {
    pub id: Uuid,
    pub device_path: String,
    pub name: String,
    pub metadata: HashMap<String, String>,
    pub timestamp: DateTime<Utc>,
    pub value: GenerationResult,
}

fn default_instance_id() -> String {
    Uuid::new_v4().to_string()
}

#[derive(Debug, Deserialize)]
pub struct OutputConfig {
    pub output_id: String,
    #[serde(default = "default_instance_id")]
    pub instance_id: String,
    #[serde(default = "RHashMap::new")]
    pub params: RHashMap<String, String>,
}

// #[sabi_trait]
pub trait OutputPlugin: Send + Sync + Debug {
    fn send(&self, payload: SensorPayload);
}

pub fn get_mock_output() -> Arc<RwLock<dyn OutputPlugin>> {
    #[derive(Debug)]
    struct Anon();
    impl OutputPlugin for Anon {
        fn send(&self, payload: SensorPayload) {
            println!("GOT = {:?}", payload);
        }
    }
    Arc::new(RwLock::new(Anon()))
}

pub type OutputPointer = Arc<RwLock<dyn OutputPlugin>>;

#[derive(Copy, Clone)]
pub struct OutputPluginDeclaration {
    pub rustc_version: &'static str,
    pub core_version: &'static str,
    pub output_id: &'static str,
    pub instance_fn: unsafe extern "C" fn(args: RHashMap<String, String>) -> OutputPointer,
}

#[macro_export]
macro_rules! export_output_plugin {
    ($output_id:expr,$instance_fn:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static PLUGIN_DECLARATION: $crate::output::OutputPluginDeclaration =
            $crate::output::OutputPluginDeclaration {
                rustc_version: $crate::RUSTC_VERSION,
                core_version: $crate::CORE_VERSION,
                output_id: $output_id,
                instance_fn: $instance_fn,
            };
    };
}
