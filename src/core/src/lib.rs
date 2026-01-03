//! Mycelix-DeSci Core
//!
//! Core functionality for Mycelix-DeSci including:
//! - Epistemic claim management with Charter v2.0 LEM Cube
//! - Multi-layer epistemic fingerprinting (LEM, Type, Quality, Network)
//! - Proof of Gradient Quality (PoGQ) for federated learning
//! - MATL (Mycelix Adaptive Trust Layer) integration
//! - Claim relationships and knowledge graph support
//! - Data verification and provenance tracking

pub mod claims;
pub mod pogq;
pub mod trust;
pub mod storage;
pub mod error;
pub mod config;
pub mod hash;
pub mod logging;
pub mod query;
pub mod utils;

// Core claim types
pub use claims::{DesciClaim, EpistemicPosition, EpistemicTier, Provenance, ClaimContent, Verification};

// Layer 1: LEM Cube (Charter v2.0)
pub use claims::{LEMCube, EmpiricalAxis, NormativeAxis, MaterialityAxis};

// Layer 3: Quality Metrics
pub use claims::QualityMetrics;

// Layer 4: Network Position
pub use claims::{NetworkPosition, ClaimRelation, ClaimRelationType};

// MATL Integration
pub use claims::MATLTrust;

// Unified Fingerprint
pub use claims::EpistemicFingerprint;

// Error handling and configuration
pub use error::{Error, Result};
pub use config::Config;
pub use query::{QueryEngine, QueryFilter};

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
