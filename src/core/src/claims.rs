//! Epistemic Claims
//!
//! Definitions and functionality for DeSci claims with epistemic tiers,
//! provenance tracking, and verification.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

/// A DeSci claim with epistemic classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesciClaim {
    /// Unique identifier for the claim
    pub id: Uuid,

    /// Epistemic tier (E0-E4)
    pub epistemic_tier: EpistemicTier,

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
    /// Create a new claim
    pub fn new(
        epistemic_tier: EpistemicTier,
        content: ClaimContent,
        creator: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            epistemic_tier,
            content,
            provenance: Vec::new(),
            creator,
            created_at: now,
            updated_at: now,
            verifications: Vec::new(),
        }
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
}
