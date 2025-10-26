// crates/satisflow-server/src/lib.rs
pub mod error;
pub mod handlers;
pub mod state;

pub use error::{AppError, Result};
pub use state::AppState;
