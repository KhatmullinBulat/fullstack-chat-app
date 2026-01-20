use std::sync::Arc;

use tokio::sync::broadcast;

use crate::models::ChatMessage;

#[derive(Clone)]
pub struct AppState {
    pub tx: broadcast::Sender<ChatMessage>,
}

impl AppState {
    pub fn new() -> Arc<Self> {
        let (tx, _) = broadcast::channel(100);
        Arc::new(Self { tx })
    }
}
