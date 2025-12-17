use tokio::sync::mpsc::{Sender, error::TrySendError};
use super::event::OperationalEvent;

#[derive(Clone)]
pub struct OperationalEmitter {
    tx: Sender<OperationalEvent>,
}

impl OperationalEmitter {
    pub fn new(tx: Sender<OperationalEvent>) -> Self {
        Self { tx }
    }

    pub fn emit(&self, event: OperationalEvent) {
        if let Err(TrySendError::Full(_)) = self.tx.try_send(event) {
            // TODO: handle it appropriately.
        }
    }
}