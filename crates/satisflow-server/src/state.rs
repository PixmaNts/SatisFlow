// crates/satisflow-server/src/state.rs
use satisflow_engine::SatisflowEngine;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub engine: Arc<RwLock<SatisflowEngine>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        Self {
            engine: Arc::new(RwLock::new(SatisflowEngine::new())),
        }
    }
}
