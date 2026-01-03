# Mycelix DeSci - Ecosystem Integration Guide

**Version**: 0.1.0
**Date**: January 3, 2026
**Status**: MVP Complete + E-N-M Integration

---

## Overview

Mycelix DeSci is now integrated into the Mycelix ecosystem with a dual epistemic classification system:

1. **E0-E4 Tiers** (Social Verification): How many independent verifications does this claim have?
2. **E-N-M Cube** (Type Classification): What kind of claim is this?

This dual system provides both **trust measurement** (E0-E4) and **claim categorization** (E-N-M).

---

## The Dual Epistemic System

### E0-E4 Verification Tiers

| Tier | Verifications Required | Description |
|------|------------------------|-------------|
| E0 | 0 | Unverified claim |
| E1 | 1 | Single-source verification |
| E2 | 2-4 | Multi-source verification |
| E3 | 5-9 | Reproducible with documented methodology |
| E4 | 10+ | Peer-reviewed and independently reproduced |

Claims **automatically upgrade** as verifications accumulate.

### E-N-M Classification Cube

Every claim is positioned in a 3D epistemic space:

```
         M (Mythic)
          │
          │    ┌─────────────┐
          │   /│            /│
          │  / │           / │
          │ /  │          /  │
          │┌───┼─────────┐   │
          │|   │         │   │
          │|   └─────────┼───┘
          │|  /          │  /
          │| /           │ /
          │|/            │/
          └┴─────────────┴────── E (Empirical)
         /
        /
       N (Normative)
```

**Dimensions**:
- **Empirical (E)**: How verifiable through observation? (0.0-1.0)
- **Normative (N)**: How aligned with ethical frameworks? (0.0-1.0)
- **Mythic (M)**: What narrative/meaning significance? (0.0-1.0)

**Examples**:
| Claim Type | E | N | M | Example |
|------------|---|---|---|---------|
| Scientific fact | 0.9 | 0.1 | 0.1 | "Water boils at 100°C at sea level" |
| Moral principle | 0.2 | 0.9 | 0.5 | "All humans have inherent dignity" |
| Origin story | 0.1 | 0.4 | 0.9 | "The universe began with the Big Bang" |
| Historical event | 0.8 | 0.5 | 0.8 | "The moon landing occurred in 1969" |

---

## Integration with Mycelix hApps

### With mycelix-knowledge

The Knowledge hApp and DeSci share the E-N-M framework:

```rust
// DeSci claim with E-N-M position
use mycelix_desci_core::{DesciClaim, EpistemicPosition, ClaimContent};

let claim = DesciClaim::with_position(
    EpistemicTier::E0,
    EpistemicPosition::scientific(0.95),  // High empirical
    content,
    "researcher@uni.edu".to_string(),
);

// Can be synced to mycelix-knowledge graph
```

### With mycelix-media

DeSci claims can be fact-checked through Media's epistemic verification:

```rust
// Scientific claim for fact-checking
let claim = DesciClaim::scientific(content, creator);
// Submit to Media hApp's factcheck zome for verification
```

### With mycelix-edunet

Research discoveries can be credentialed through EduNet:

```rust
// Research achievement becomes W3C Verifiable Credential
// DeSci E4 claim -> EduNet credential_zome -> W3C VC
```

---

## API Usage

### Creating Claims

```rust
use mycelix_desci_core::{
    DesciClaim, EpistemicTier, EpistemicPosition, ClaimContent
};

// Scientific claim (high empirical)
let claim = DesciClaim::scientific(content, "researcher@uni.edu".to_string());

// Ethical claim (high normative)
let claim = DesciClaim::ethical(content, "ethicist@uni.edu".to_string());

// Narrative claim (high mythic)
let claim = DesciClaim::narrative(content, "historian@uni.edu".to_string());

// Custom position
let position = EpistemicPosition::new(0.7, 0.6, 0.3);
let claim = DesciClaim::with_position(
    EpistemicTier::E1,
    position,
    content,
    "creator@uni.edu".to_string(),
);
```

### Querying by E-N-M Position

```rust
// Find all highly empirical claims
let scientific_claims = query_engine
    .query(QueryFilter::new()
        .with_min_empirical(0.8))
    .await?;

// Find balanced claims
let balanced = claims.iter()
    .filter(|c| c.epistemic_position.is_balanced())
    .collect();
```

---

## Workspace Location

```
mycelix-workspace/happs/desci -> /srv/luminous-dynamics/mycelix-desci/
```

---

## Building & Testing

```bash
cd mycelix-desci

# Using Nix (recommended)
nix develop
cargo test --lib

# Run specific tests
cargo test claims::tests::

# Run benchmarks
cargo bench

# Run CLI
cargo run --bin mycelix -- --help

# Run API server
cargo run --bin mycelix-api
```

---

## Future: Holochain Integration

The DeSci core is currently standalone Rust for maximum performance. Future plans include:

1. **Storage Backend**: Replace MemoryStorage with Holochain DHT
2. **Identity**: Integrate DID:mycelix for researcher identity
3. **Trust**: Connect to MATL for reputation-weighted verification
4. **Bridge**: Enable cross-hApp claim referencing

---

## Contributing

See `CONTRIBUTING.md` for guidelines.

---

*Part of the Mycelix Civilizational OS - Knowledge Pillar*
