use config::{Config, Environment, File, Source};
use home::home_dir;
use serde::Deserialize;

use iot_simulator_api::generator::GeneratorType;

const CONFIG_NAME: &str = ".iot-sim-config.toml";
const CONFIG_ENV_PREFIX: &str = "IOT_SIM_";

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GeneratorPluginConf {
    pub(crate) id: String,
    pub(crate) generator_type: GeneratorType,
    pub(crate) return_type: Option<String>,
    pub(crate) path: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct OutputPluginConf {
    pub(crate) id: String,
    pub(crate) path: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    #[serde(default = "Vec::new")]
    pub generator_plugins: Vec<GeneratorPluginConf>,
    #[serde(default = "Vec::new")]
    pub output_plugins: Vec<OutputPluginConf>,
}

pub fn load_settings(settings_file: Option<String>) -> Settings {
    let config_builder = Config::builder();

    let mut sources: Vec<Box<dyn Source + Send + Sync>> = Vec::new();

    if let Some(path) = home_dir() {
        sources.push(Box::new(
            File::with_name(&format!("{}/{}", path.as_path().display(), CONFIG_NAME))
                .required(false),
        ));
    }

    sources.push(Box::new(File::with_name(CONFIG_NAME).required(false)));
    sources.push(Box::new(Environment::with_prefix(CONFIG_ENV_PREFIX)));
    if let Some(settings) = settings_file {
        sources.push(Box::new(File::with_name(settings.as_str()).required(true)));
    }

    config_builder
        .add_source(sources)
        .build()
        .expect("Failed to load config")
        .try_deserialize()
        .expect("Failed to deserialize config")
}
