use futures_util::future::BoxFuture;
use futures_util::TryFutureExt;
use tokio::sync::broadcast;
use tokio::sync::broadcast::{Receiver, Sender};

use crate::output::SensorPayload;

type SimSenderResult<'a> = BoxFuture<'a, Result<(), String>>;
type SimReceiverResult<'a> = BoxFuture<'a, Result<SensorPayload, String>>;

pub trait SimSender {
    fn send(&self, payload: SensorPayload) -> BoxFuture<Result<(), String>>;
}

pub trait SimReceiver {
    fn next(&mut self) -> SimReceiverResult;
}

pub struct SimChannel {
    pub tx: Box<dyn SimSender + Send + Sync>,
    pub rx: Box<dyn SimReceiver + Send + Sync>,
}

pub trait ChannelPlugin {
    fn init() -> SimChannel;
    fn with_capacity(capacity: usize) -> SimChannel;
}

pub struct DefaultChannel;
impl ChannelPlugin for DefaultChannel {
    fn init() -> SimChannel {
        DefaultChannel::with_capacity(65536)
    }

    fn with_capacity(capacity: usize) -> SimChannel {
        let (tx, rx) = broadcast::channel(capacity);
        SimChannel {
            tx: Box::new(DefaultSender(tx)),
            rx: Box::new(DefaultReceiver(rx)),
        }
    }
}

struct DefaultSender(Sender<SensorPayload>);
impl SimSender for DefaultSender {
    fn send(&self, payload: SensorPayload) -> SimSenderResult {
        Box::pin(async {
            self.0.send(payload).unwrap();
            Ok(())
        })
    }
}

struct DefaultReceiver(Receiver<SensorPayload>);
impl SimReceiver for DefaultReceiver {
    fn next(&mut self) -> SimReceiverResult {
        Box::pin(self.0.recv().map_err(|e| e.to_string()))
    }
}
