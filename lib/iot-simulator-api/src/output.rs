use std::collections::HashMap;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use tokio::sync::broadcast::Receiver;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::generator::GenerationResult;

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct SensorPayload {
    pub id: Uuid,
    pub device_path: String,
    pub name: String,
    pub metadata: HashMap<String, String>,
    pub timestamp: DateTime<Utc>,
    pub value: GenerationResult,
}

pub trait OutputPlugin {
    fn register(&self, payload_receiver: &'static mut Receiver<SensorPayload>);
}

pub fn get_mock_output() -> Arc<RwLock<dyn OutputPlugin>> {
    #[derive(Debug)]
    struct Anon();
    impl OutputPlugin for Anon {
        fn register(&self, payload_receiver: &'static mut Receiver<SensorPayload>) {
            tokio::spawn(async move {
                while let Ok(message) = &payload_receiver.recv().await {
                    println!("GOT = {:?}", message);
                }
            });
        }
    }
    Arc::new(RwLock::new(Anon()))
}
