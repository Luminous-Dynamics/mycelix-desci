//! Epistemic Claims
//!
//! Definitions and functionality for DeSci claims with epistemic tiers,
//! provenance tracking, and verification.
//!
//! This module implements a dual epistemic classification system:
//! - **E0-E4 Tiers**: Verification-based trust levels (social consensus)
//! - **E-N-M Cube**: Three-dimensional epistemic classification:
//!   - E (Empirical): How verifiable through observation? (0.0-1.0)
//!   - N (Normative): How aligned with ethical frameworks? (0.0-1.0)
//!   - M (Mythic): What narrative/meaning significance? (0.0-1.0)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Three-dimensional epistemic position in the E-N-M cube
///
/// Every claim is positioned in a 3D space measuring:
/// - **Empirical (E)**: How verifiable through observation/experiment (0.0-1.0)
/// - **Normative (N)**: How aligned with ethical/value frameworks (0.0-1.0)
/// - **Mythic (M)**: What narrative/meaning significance it holds (0.0-1.0)
///
/// # Examples
/// - Scientific fact: High E (0.9), Low N (0.1), Low M (0.1)
/// - Moral principle: Low E (0.2), High N (0.9), Variable M (0.5)
/// - Origin story: Low E (0.1), Variable N (0.4), High M (0.9)
/// - Historical event: High E (0.8), Variable N (0.5), High M (0.8)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct EpistemicPosition {
    /// Empirical dimension: How verifiable through observation? (0.0-1.0)
    pub empirical: f64,
    /// Normative dimension: How aligned with ethical frameworks? (0.0-1.0)
    pub normative: f64,
    /// Mythic dimension: What narrative/meaning significance? (0.0-1.0)
    pub mythic: f64,
}

impl EpistemicPosition {
    /// Create a new epistemic position
    pub fn new(empirical: f64, normative: f64, mythic: f64) -> Self {
        Self {
            empirical: empirical.clamp(0.0, 1.0),
            normative: normative.clamp(0.0, 1.0),
            mythic: mythic.clamp(0.0, 1.0),
        }
    }

    /// Create position for a scientific/empirical claim
    pub fn scientific(empirical: f64) -> Self {
        Self::new(empirical, 0.1, 0.1)
    }

    /// Create position for an ethical/normative claim
    pub fn ethical(normative: f64) -> Self {
        Self::new(0.2, normative, 0.3)
    }

    /// Create position for a narrative/mythic claim
    pub fn narrative(mythic: f64) -> Self {
        Self::new(0.1, 0.3, mythic)
    }

    /// Compute Euclidean distance to another position
    pub fn distance(&self, other: &EpistemicPosition) -> f64 {
        let de = self.empirical - other.empirical;
        let dn = self.normative - other.normative;
        let dm = self.mythic - other.mythic;
        (de * de + dn * dn + dm * dm).sqrt()
    }

    /// Get the dominant dimension
    pub fn dominant_dimension(&self) -> &'static str {
        if self.empirical >= self.normative && self.empirical >= self.mythic {
            "empirical"
        } else if self.normative >= self.empirical && self.normative >= self.mythic {
            "normative"
        } else {
            "mythic"
        }
    }

    /// Check if this is a balanced claim (all dimensions within 0.3 of each other)
    pub fn is_balanced(&self) -> bool {
        let max = self.empirical.max(self.normative).max(self.mythic);
        let min = self.empirical.min(self.normative).min(self.mythic);
        max - min <= 0.3
    }
}

impl Default for EpistemicPosition {
    fn default() -> Self {
        Self::new(0.5, 0.5, 0.5)
    }
}

/// Epistemic tier classification (E0-E4)
///
/// Based on Mycelix epistemic framework:
/// - E0: Unverified claim
/// - E1: Single-source verification
/// - E2: Multi-source verification
/// - E3: Reproducible with documented methodology
/// - E4: Peer-reviewed and independently reproduced
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum EpistemicTier {
    E0 = 0,
    E1 = 1,
    E2 = 2,
    E3 = 3,
    E4 = 4,
}

impl EpistemicTier {
    /// Returns a human-readable description of the tier
    pub fn description(&self) -> &'static str {
        match self {
            EpistemicTier::E0 => "Unverified claim",
            EpistemicTier::E1 => "Single-source verification",
            EpistemicTier::E2 => "Multi-source verification",
            EpistemicTier::E3 => "Reproducible with documented methodology",
            EpistemicTier::E4 => "Peer-reviewed and independently reproduced",
        }
    }

    /// Minimum number of verifications required for this tier
    pub fn min_verifications(&self) -> usize {
        match self {
            EpistemicTier::E0 => 0,
            EpistemicTier::E1 => 1,
            EpistemicTier::E2 => 2,
            EpistemicTier::E3 => 3,
            EpistemicTier::E4 => 5,
        }
    }
}

/// Provenance information for a claim
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provenance {
    /// Source identifier (e.g., "PubChem ID:123", "DOI:10.1234/example")
    pub source: String,

    /// Type of source (e.g., "database", "publication", "repository")
    pub source_type: String,

    /// URL or URI for accessing the source
    pub url: Option<String>,

    /// Timestamp when the claim was created from this source
    pub timestamp: DateTime<Utc>,

    /// Additional metadata
    pub metadata: serde_json::Value,
}

impl Provenance {
    /// Create a new provenance entry
    pub fn new(source: String, source_type: String) -> Self {
        Self {
            source,
            source_type,
            url: None,
            timestamp: Utc::now(),
            metadata: serde_json::json!({}),
        }
    }

    /// Set the URL for this provenance
    pub fn with_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: &str, value: serde_json::Value) -> Self {
        if let Some(obj) = self.metadata.as_object_mut() {
            obj.insert(key.to_string(), value);
        }
        self
    }
}

/// Content of a DeSci claim
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimContent {
    /// Hash of the dataset or content (e.g., SHA-256)
    pub dataset_hash: String,

    /// Description of the claim
    pub description: String,

    /// Category (e.g., "genomics", "longevity", "climate")
    pub category: String,

    /// Keywords for searchability
    pub keywords: Vec<String>,

    /// IPFS CID or other storage reference
    pub storage_ref: Option<String>,

    /// Reproducibility score (0.0 - 1.0)
    pub reproducibility_score: Option<f64>,

    /// License (e.g., "CC-BY-4.0", "MIT")
    pub license: Option<String>,
}

/// A DeSci claim with dual epistemic classification
///
/// Combines two complementary epistemic systems:
/// - **E0-E4 Tiers**: Social verification levels (how many independent verifications)
/// - **E-N-M Position**: Type classification (empirical vs normative vs mythic)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesciClaim {
    /// Unique identifier for the claim
    pub id: Uuid,

    /// Epistemic tier (E0-E4) - verification-based trust level
    pub epistemic_tier: EpistemicTier,

    /// Epistemic position in E-N-M cube - type classification
    pub epistemic_position: EpistemicPosition,

    /// Content of the claim
    pub content: ClaimContent,

    /// Provenance chain (multiple sources)
    pub provenance: Vec<Provenance>,

    /// Creator's public key or DID
    pub creator: String,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,

    /// Verification signatures
    pub verifications: Vec<Verification>,
}

/// A verification signature for a claim
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verification {
    /// Verifier's public key or DID
    pub verifier: String,

    /// Timestamp of verification
    pub timestamp: DateTime<Utc>,

    /// Signature (cryptographic proof)
    pub signature: Vec<u8>,

    /// Notes from the verifier
    pub notes: Option<String>,
}

impl DesciClaim {
    /// Create a new claim with default epistemic position
    pub fn new(
        epistemic_tier: EpistemicTier,
        content: ClaimContent,
        creator: String,
    ) -> Self {
        Self::with_position(epistemic_tier, EpistemicPosition::default(), content, creator)
    }

    /// Create a new claim with specific E-N-M position
    pub fn with_position(
        epistemic_tier: EpistemicTier,
        epistemic_position: EpistemicPosition,
        content: ClaimContent,
        creator: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            epistemic_tier,
            epistemic_position,
            content,
            provenance: Vec::new(),
            creator,
            created_at: now,
            updated_at: now,
            verifications: Vec::new(),
        }
    }

    /// Create a scientific claim (high empirical)
    pub fn scientific(content: ClaimContent, creator: String) -> Self {
        Self::with_position(
            EpistemicTier::E0,
            EpistemicPosition::scientific(0.9),
            content,
            creator,
        )
    }

    /// Create an ethical claim (high normative)
    pub fn ethical(content: ClaimContent, creator: String) -> Self {
        Self::with_position(
            EpistemicTier::E0,
            EpistemicPosition::ethical(0.9),
            content,
            creator,
        )
    }

    /// Create a narrative claim (high mythic)
    pub fn narrative(content: ClaimContent, creator: String) -> Self {
        Self::with_position(
            EpistemicTier::E0,
            EpistemicPosition::narrative(0.9),
            content,
            creator,
        )
    }

    /// Add provenance to the claim
    pub fn add_provenance(&mut self, prov: Provenance) {
        self.provenance.push(prov);
        self.updated_at = Utc::now();
    }

    /// Add a verification
    pub fn add_verification(&mut self, verification: Verification) {
        self.verifications.push(verification);
        self.updated_at = Utc::now();

        // Potentially upgrade tier based on verifications
        self.update_tier_from_verifications();
    }

    /// Update epistemic tier based on number of verifications
    fn update_tier_from_verifications(&mut self) {
        let verification_count = self.verifications.len();

        // Determine the highest tier we can achieve
        let potential_tier = match verification_count {
            0 => EpistemicTier::E0,
            1 => EpistemicTier::E1,
            2..=4 => EpistemicTier::E2,
            5..=9 => EpistemicTier::E3,
            _ => EpistemicTier::E4,
        };

        // Only upgrade, never downgrade automatically
        if potential_tier > self.epistemic_tier {
            self.epistemic_tier = potential_tier;
        }
    }

    /// Check if the claim meets the minimum requirements for its tier
    pub fn is_valid_for_tier(&self) -> bool {
        let min_verifications = self.epistemic_tier.min_verifications();
        self.verifications.len() >= min_verifications
    }

    /// Serialize to JSON
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize from JSON
    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epistemic_tier_ordering() {
        assert!(EpistemicTier::E4 > EpistemicTier::E0);
        assert!(EpistemicTier::E3 > EpistemicTier::E2);
    }

    #[test]
    fn test_create_claim() {
        let content = ClaimContent {
            dataset_hash: "abc123".to_string(),
            description: "Test dataset".to_string(),
            category: "genomics".to_string(),
            keywords: vec!["test".to_string()],
            storage_ref: None,
            reproducibility_score: Some(0.95),
            license: Some("MIT".to_string()),
        };

        let claim = DesciClaim::new(
            EpistemicTier::E0,
            content,
            "creator_pubkey".to_string(),
        );

        assert_eq!(claim.epistemic_tier, EpistemicTier::E0);
        assert_eq!(claim.verifications.len(), 0);
    }

    #[test]
    fn test_provenance() {
        let prov = Provenance::new(
            "PubChem ID:123".to_string(),
            "database".to_string(),
        )
        .with_url("https://pubchem.ncbi.nlm.nih.gov/compound/123".to_string());

        assert_eq!(prov.source, "PubChem ID:123");
        assert!(prov.url.is_some());
    }

    #[test]
    fn test_epistemic_position_new() {
        let pos = EpistemicPosition::new(0.9, 0.5, 0.2);
        assert_eq!(pos.empirical, 0.9);
        assert_eq!(pos.normative, 0.5);
        assert_eq!(pos.mythic, 0.2);
    }

    #[test]
    fn test_epistemic_position_clamping() {
        let pos = EpistemicPosition::new(1.5, -0.3, 0.5);
        assert_eq!(pos.empirical, 1.0);
        assert_eq!(pos.normative, 0.0);
        assert_eq!(pos.mythic, 0.5);
    }

    #[test]
    fn test_epistemic_position_scientific() {
        let pos = EpistemicPosition::scientific(0.95);
        assert_eq!(pos.empirical, 0.95);
        assert!(pos.normative < 0.2);
        assert!(pos.mythic < 0.2);
        assert_eq!(pos.dominant_dimension(), "empirical");
    }

    #[test]
    fn test_epistemic_position_ethical() {
        let pos = EpistemicPosition::ethical(0.9);
        assert_eq!(pos.normative, 0.9);
        assert_eq!(pos.dominant_dimension(), "normative");
    }

    #[test]
    fn test_epistemic_position_narrative() {
        let pos = EpistemicPosition::narrative(0.85);
        assert_eq!(pos.mythic, 0.85);
        assert_eq!(pos.dominant_dimension(), "mythic");
    }

    #[test]
    fn test_epistemic_position_distance() {
        let pos1 = EpistemicPosition::new(0.0, 0.0, 0.0);
        let pos2 = EpistemicPosition::new(1.0, 0.0, 0.0);
        assert!((pos1.distance(&pos2) - 1.0).abs() < 0.001);

        let pos3 = EpistemicPosition::new(1.0, 1.0, 1.0);
        let expected_distance = (3.0_f64).sqrt();
        assert!((pos1.distance(&pos3) - expected_distance).abs() < 0.001);
    }

    #[test]
    fn test_epistemic_position_balanced() {
        let balanced = EpistemicPosition::new(0.5, 0.6, 0.7);
        assert!(balanced.is_balanced());

        let unbalanced = EpistemicPosition::new(0.1, 0.5, 0.9);
        assert!(!unbalanced.is_balanced());
    }

    #[test]
    fn test_create_scientific_claim() {
        let content = ClaimContent {
            dataset_hash: "hash123".to_string(),
            description: "Research finding".to_string(),
            category: "biology".to_string(),
            keywords: vec!["research".to_string()],
            storage_ref: None,
            reproducibility_score: Some(0.9),
            license: Some("CC-BY-4.0".to_string()),
        };

        let claim = DesciClaim::scientific(content, "researcher@uni.edu".to_string());

        assert_eq!(claim.epistemic_tier, EpistemicTier::E0);
        assert_eq!(claim.epistemic_position.dominant_dimension(), "empirical");
        assert!(claim.epistemic_position.empirical > 0.8);
    }

    #[test]
    fn test_create_claim_with_position() {
        let content = ClaimContent {
            dataset_hash: "hash456".to_string(),
            description: "Mixed claim".to_string(),
            category: "philosophy".to_string(),
            keywords: vec!["ethics".to_string()],
            storage_ref: None,
            reproducibility_score: None,
            license: None,
        };

        let position = EpistemicPosition::new(0.3, 0.8, 0.4);
        let claim = DesciClaim::with_position(
            EpistemicTier::E1,
            position,
            content,
            "philosopher@uni.edu".to_string(),
        );

        assert_eq!(claim.epistemic_tier, EpistemicTier::E1);
        assert_eq!(claim.epistemic_position.normative, 0.8);
        assert_eq!(claim.epistemic_position.dominant_dimension(), "normative");
    }
}
