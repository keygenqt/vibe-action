//! Validation trait and utilities.

use anyhow::Result;

pub mod action;
pub mod app;
pub mod arg;
pub mod cluster;
pub mod flow;
pub mod flows;

/// Trait for types that can be validated.
pub trait ValidateTrait {
    fn validate(&self) -> Result<()>;
}
