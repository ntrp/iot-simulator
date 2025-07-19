use tokio::sync::broadcast::{self, error::SendError};
use tokio::sync::broadcast::Sender;

use tokio::task::JoinHandle;

use crate::output::{OutputPointer, SensorPayload};

pub trait ChannelPlugin {
    fn send(&mut self, payload: SensorPayload) -> Result<usize, SendError<SensorPayload>>;
    fn subscribe(&mut self, output: OutputPointer);
}

pub struct InMemoryChannel {
    tx: Sender<SensorPayload>,
    rx_handlers: Vec<JoinHandle<()>>,
}

impl InMemoryChannel {
    pub fn new() -> InMemoryChannel {
        let (tx, _) = broadcast::channel(65535);
        InMemoryChannel {
            tx,
            rx_handlers: vec![]
        }
    }
}

impl ChannelPlugin for InMemoryChannel {
    fn send(&mut self, payload: SensorPayload) -> Result<usize, SendError<SensorPayload>> {
        self.tx.send(payload)
    }

    fn subscribe(&mut self, output: OutputPointer) {
        let mut rx = self.tx.subscribe();
        let handle = tokio::spawn(async move {
            while let Ok(result) = rx.recv().await {
                output.to_owned().write().expect("Cannot get write lock").send(result);
            }
        });
        self.rx_handlers.push(handle);
    }
}
