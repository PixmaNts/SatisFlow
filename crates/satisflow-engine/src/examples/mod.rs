//! Examples module for the Satisflow engine
//!
//! This module contains example implementations and demonstrations
//! of how to use the Satisflow engine.

pub mod factory_example;
pub mod example_usage;
pub mod test_program;

// Re-export the main example functions for easy access
pub use factory_example::create_sample_factory_setup;
pub use example_usage::run_factory_demo;
pub use test_program::run_test_program;