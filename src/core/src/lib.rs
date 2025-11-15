//! Mycelix-DeSci Core
//!
//! Core functionality for Mycelix-DeSci including:
//! - Epistemic claim management (E0-E4 tiers)
//! - Proof of Gradient Quality (PoGQ) for federated learning
//! - MATL (Mycelix Adaptive Trust Layer) integration
//! - Data verification and provenance tracking

pub mod claims;
pub mod pogq;
pub mod trust;
pub mod storage;
pub mod error;
pub mod config;
pub mod hash;
pub mod logging;

pub use claims::{DesciClaim, EpistemicTier, Provenance};
pub use error::{Error, Result};
pub use config::Config;

/// Version of the Mycelix-DeSci protocol
pub const PROTOCOL_VERSION: &str = "0.1.0";

/// Default Byzantine fault tolerance threshold for PoGQ (45%)
pub const DEFAULT_BFT_THRESHOLD: f64 = 0.45;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_version() {
        assert_eq!(PROTOCOL_VERSION, "0.1.0");
    }
}
