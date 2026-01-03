//! Mycelix-DeSci Core
//!
//! Core functionality for Mycelix-DeSci including:
//! - Epistemic claim management with Charter v2.0 LEM Cube
//! - Multi-layer epistemic fingerprinting (LEM, Type, Quality, Network)
//! - Proof of Gradient Quality (PoGQ) for federated learning
//! - MATL (Mycelix Adaptive Trust Layer) integration
//! - Claim relationships and knowledge graph support
//! - Data verification and provenance tracking
//! - Claim evolution and versioning (Constitution Schema v2.0)
//! - Dispute resolution system (Epistemic Charter §5)
//! - Cartel detection for anti-collusion
//! - Reproducibility tracking for scientific claims
//! - Prediction markets for epistemic outcomes
//! - Semantic similarity detection
//! - Time-based decay mechanics

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

// New epistemic modules
pub mod evolution;
pub mod dispute;
pub mod cartel;
pub mod reproducibility;
pub mod prediction;
pub mod semantic;
pub mod decay;

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

// Claim Evolution (Constitution Schema v2.0)
pub use evolution::{ClaimEvolution, EvolutionType, ClaimStatus};

// Dispute Resolution (Epistemic Charter §5)
pub use dispute::{
    Dispute, DisputeStatus, ChallengeType, Resolution, ResolutionOutcome,
    Evidence, EvidenceType, AuthorResponse, RequiredAction, ActionType,
};

// Cartel Detection (Anti-Collusion)
pub use cartel::{
    CartelDetector, CartelDetectionConfig, CartelDetectionResult,
    CartelPattern, CartelRecommendation, VerificationEvent,
};

// Reproducibility Tracking
pub use reproducibility::{
    ReplicationAttempt, ReplicationStatus, ReplicationOutcome,
    MethodologyMatch, ReproducibilityStats, ReproducibilityRegistry,
};

// Prediction Markets
pub use prediction::{
    PredictionMarket, PredictionMarketRegistry, MarketType, MarketState,
    Position, Settlement, MarketError, ResolutionMethod,
};

// Semantic Similarity
pub use semantic::{
    SimilarityEngine, SimilarityScore, SimilarityComponents,
    SimilarityRelationship, DuplicateCheckResult, DuplicateRecommendation,
    ClaimContent as SemanticClaimContent,
};

// Decay Mechanics
pub use decay::{
    DecayCalculator, DecayConfig, DecayFunction, DecayStats,
    DecayingAccumulator,
};

// Error handling and configuration
pub use error::{Error, Result};
pub use config::Config;
pub use query::{QueryEngine, QueryFilter};

/// Version of the Mycelix-DeSci protocol
pub const PROTOCOL_VERSION: &str = "0.2.0";

/// Default Byzantine fault tolerance threshold for PoGQ (45%)
pub const DEFAULT_BFT_THRESHOLD: f64 = 0.45;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_version() {
        assert_eq!(PROTOCOL_VERSION, "0.2.0");
    }
}
