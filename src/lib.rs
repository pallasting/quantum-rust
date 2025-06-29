//! Quantum Rust - The World's First Quantum-Enhanced Systems Programming Language
//!
//! This crate provides quantum-enhanced compilation and runtime features
//! while maintaining 100% compatibility with existing Rust code.

pub mod quantum;
pub mod arrow;
pub mod compiler;

pub use quantum::prelude::*;

/// Quantum Rust version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Check if quantum features are enabled
pub fn is_quantum_enabled() -> bool {
    cfg!(feature = "quantum-core")
}
