// crates/satisflow-server/src/state.rs
use std::sync::Arc;
use tokio::sync::RwLock;
use satisflow_engine::SatisflowEngine;

#[derive(Clone)]
pub struct AppState {
    pub engine: Arc<RwLock<SatisflowEngine>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            engine: Arc::new(RwLock::new(SatisflowEngine::new())),
        }
    }
}