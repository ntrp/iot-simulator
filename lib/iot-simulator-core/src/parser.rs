extern crate pest;

use std::collections::HashMap;
use std::fs;

use chrono::{DateTime, Duration, Utc};
use derivative::Derivative;
use pest::iterators::Pair;
use pest::Parser;
use serde::{Deserialize, Deserializer};
use uuid::Uuid;

use iot_simulator_api::generator::{GeneratorConfig, GeneratorPointer};

use crate::plugin::GeneratorPluginRegistry;

pub fn parse_simulation(file_path: String) -> Simulation {
    let config = fs::read_to_string(&file_path)
        .unwrap_or_else(|_| panic!("Failed to open file {}", &file_path));
    match ron::from_str(config.as_str()) {
        Ok(res) => res,
        Err(error) => panic!("{:?}", error),
    }
}
#[derive(Derivative, Deserialize)]
#[derivative(Debug)]
pub struct Sensor {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    pub name: String,
    #[serde(default = "HashMap::new")]
    pub metadata: HashMap<String, String>,
    pub sampling_rate: i64,
    #[serde(deserialize_with = "to_generator")]
    #[derivative(Debug = "ignore")]
    pub value_generator: GeneratorPointer,
}

fn to_generator<'de, D>(deserializer: D) -> Result<GeneratorPointer, D::Error>
where
    D: Deserializer<'de>,
{
    let config = GeneratorConfig::deserialize(deserializer)?;
    let generator = GeneratorPluginRegistry::register(&config);
    Ok(generator)
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Device {
    #[serde(default = "Uuid::new_v4")]
    pub(crate) id: Uuid,
    pub(crate) name: String,
    #[serde(default = "HashMap::new")]
    pub(crate) metadata: HashMap<String, String>,
    pub(crate) sensors: Vec<Sensor>,
    #[serde(default = "Vec::new")]
    pub(crate) devices: Vec<Device>,
}

#[derive(Parser)]
#[grammar = "time.pest"]
pub struct TimeUnitParser;

#[derive(Debug, Deserialize)]
enum DT {
    Now(),
    Utc(String),
    Offset(String),
    Never(),
}

fn to_date_time_start_at<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let holder = DT::deserialize(deserializer)?;
    Ok(match_dt(holder, true, "start_at"))
}

fn to_date_time_end_at<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let holder = DT::deserialize(deserializer)?;
    Ok(match_dt(holder, false, "end_at"))
}

fn match_dt(holder: DT, forbid_never: bool, field: &str) -> DateTime<Utc> {
    match holder {
        DT::Now() => Utc::now(),
        DT::Utc(val) => DateTime::parse_from_rfc3339(&val)
            .unwrap()
            .with_timezone(&Utc),
        DT::Offset(val) => {
            let mut pairs =
                TimeUnitParser::parse(Rule::date_time, &val).unwrap_or_else(|e| panic!("{}", e));
            let pair = pairs.next().unwrap();
            let mut minus: bool = false;
            let mut seconds = 0;
            for inner_pair in pair.into_inner() {
                match inner_pair.as_rule() {
                    Rule::minus => minus = true,
                    Rule::year => seconds += 365 * 24 * 60 * 60 * get_value(inner_pair),
                    Rule::month => seconds += 30 * 24 * 60 * 60 * get_value(inner_pair),
                    Rule::day => seconds += 24 * 60 * 60 * get_value(inner_pair),
                    Rule::hour => seconds += 60 * 60 * get_value(inner_pair),
                    Rule::minute => seconds += 60 * get_value(inner_pair),
                    Rule::second => seconds += get_value(inner_pair),
                    _ => unreachable!(),
                };
            }
            let total_duration = Duration::seconds(seconds);
            if minus {
                Utc::now() - total_duration
            } else {
                Utc::now() + total_duration
            }
        }
        DT::Never() => {
            if forbid_never {
                panic!("Never is not allowed for the field {}", field)
            } else {
                Utc::now() + Duration::days(365 * 100)
            }
        }
    }
}

fn get_value(pair: Pair<Rule>) -> i64 {
    pair.into_inner()
        .as_str()
        .parse::<i64>()
        .unwrap_or_else(|e| panic!("Unable to parse value to int: {}", e))
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Simulation {
    name: String,
    #[serde(default = "String::new")]
    description: String,
    #[serde(deserialize_with = "to_date_time_start_at")]
    pub(crate) start_at: DateTime<Utc>,
    #[serde(deserialize_with = "to_date_time_end_at")]
    pub(crate) end_at: DateTime<Utc>,
    pub(crate) devices: Vec<Device>,
    #[serde(default = "Vec::new")]
    output_plugins: Vec<String>,
}
