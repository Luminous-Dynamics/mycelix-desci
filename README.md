# Mycelix-DeSci

> Verifiable, Privacy-Preserving Infrastructure for Decentralized Science

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Status: Alpha](https://img.shields.io/badge/Status-Alpha-orange.svg)]()

## Overview

Mycelix-DeSci extends the [Mycelix Protocol](https://github.com/luminousdynamics/mycelix-core) to enable **verifiable data sharing**, **federated research**, and **IP tokenization** in scientific workflows. By leveraging the Mycelix Adaptive Trust Layer (MATL) and advanced cryptographic techniques, we're building infrastructure to democratize access to research data, enhance reproducibility, and integrate with the broader DeSci ecosystem.

### Key Features

- **🔬 Verifiable Data Sharing**: Cryptographically proven datasets with epistemic tier classifications (E0-E4)
- **🤝 Federated Research**: Byzantine-resistant federated learning (up to 45% tolerance via Proof of Gradient Quality)
- **💎 IP Tokenization**: RWA framework for research outputs, compatible with existing IP-NFT standards
- **🌐 DeSci Integrations**: Native support for VitaDAO, Molecule, DeSci Labs, and other ecosystem projects
- **🔐 Privacy-Preserving**: Adaptive differential privacy for sensitive biomedical data
- **🧠 Decentralized Knowledge Graph**: Semantic queries across distributed research claims

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Mycelix-DeSci Layer                       │
├─────────────────────────────────────────────────────────────┤
│  DeSci Features │ IP-NFTs │ Fed Learning │ Data Verification│
├─────────────────────────────────────────────────────────────┤
│              Mycelix Core Protocol (MATL)                    │
├─────────────────────────────────────────────────────────────┤
│  DHT (Holochain) │ DKG │ zk-STARKs │ IBC Bridges           │
└─────────────────────────────────────────────────────────────┘
```

### Technical Stack

- **Backend**: Rust (core/MATL/PoGQ), Python (ML/federated learning)
- **Frontend**: TypeScript, Svelte
- **Blockchain**: zk-STARKs (Risc0), IBC (Cosmos/Ethereum bridges)
- **Storage**: IPFS, Filecoin
- **ML/FL**: PyTorch, Flower, BioPython

## Quick Start

### Prerequisites

- Rust 1.75+ (`rustup`)
- Python 3.11+
- Node.js 20+
- Docker (optional)

### Installation

```bash
# Clone the repository
git clone https://github.com/luminousdynamics/mycelix-desci.git
cd mycelix-desci

# Install Rust dependencies
cargo build --release

# Install Python dependencies
cd ml
python -m venv venv
source venv/bin/activate  # or `venv\Scripts\activate` on Windows
pip install -r requirements.txt

# Install frontend dependencies
cd ../frontend
npm install
```

### Basic Usage

```bash
# Initialize configuration
mycelix-desci init

# Upload a dataset
mycelix-desci upload dataset.csv \
  --tier E2 \
  --category genomics \
  --description "CRISPR gene editing results" \
  --provenance "Lab Notebook:2024-001"

# Query claims
mycelix-desci query --category longevity --min-tier E3

# Verify a claim
mycelix-desci verify <CLAIM-ID> --file dataset.csv

# Calculate file hash
mycelix-desci hash dataset.csv
```

See [CLI Usage Guide](docs/guides/cli-usage.md) for complete documentation.

## Project Structure

```
mycelix-desci/
├── src/
│   ├── core/           # Rust core (MATL, PoGQ, DHT integration)
│   ├── ml/             # Python federated learning modules
│   └── contracts/      # Smart contracts (IP-NFT, governance)
├── frontend/           # Svelte UI for data queries and management
├── examples/           # Example integrations and POCs
├── docs/               # Documentation and guides
├── tests/              # Integration and unit tests
└── scripts/            # Build and deployment scripts
```

## Roadmap

### Phase 1: Foundation (Q4 2025 - Q1 2026)
- ✅ Repository setup and core infrastructure
- 🔄 MVP for verifiable data sharing
- 🔄 Basic PoGQ implementation for federated learning
- 🔄 Initial UI for DKG queries

### Phase 2: Integrations (Q2 - Q4 2026)
- IP tokenization framework
- VitaDAO, Molecule, DeSci Labs integrations
- Advanced federated learning for biomedical data
- Security audit

### Phase 3: Scaling (2027+)
- Mainnet launch
- Community governance DAO
- Ecosystem expansion
- Continuous optimization

See [ROADMAP.md](docs/ROADMAP.md) for detailed timelines and milestones.

## Use Cases

### 1. Verifiable Dataset Sharing
Researchers upload datasets with cryptographic proofs and epistemic classifications, ensuring transparency and reproducibility.

### 2. Federated Biomedical Research
Multiple institutions collaboratively train ML models on sensitive genomics data without exposing raw data, using PoGQ for Byzantine fault tolerance.

### 3. Tokenized Research IP
Convert research outputs (papers, datasets, models) into tradeable IP-NFTs, enabling new funding mechanisms for open science.

### 4. Decentralized Clinical Trials
Coordinate multi-site clinical trials with verifiable data collection and privacy-preserving analytics.

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Workflow

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes and add tests
4. Run tests (`cargo test && pytest && npm test`)
5. Commit with descriptive messages
6. Push and open a Pull Request

### Community

- **GitHub Discussions**: For questions and ideas
- **Issues**: Bug reports and feature requests
- **Discord**: [Join our server](https://discord.gg/mycelix) (coming soon)

## Security

- **Audits**: Planned Q4 2026 with external security firm
- **Bug Bounties**: Coming in Phase 2
- **Responsible Disclosure**: security@mycelix.org (coming soon)

For security-sensitive issues, please see [SECURITY.md](SECURITY.md).

## License

This project is licensed under the MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

- Built on the [Mycelix Protocol](https://github.com/luminousdynamics/mycelix-core)
- Inspired by [awesome-desci](https://github.com/DeSciWorldDAO/awesome-desci)
- Integrations with VitaDAO, Molecule, DeSci Labs, and the broader DeSci community

## Resources

- **Documentation**: [docs/](docs/)
- **Examples**: [examples/](examples/)
- **Research Papers**: [docs/research/](docs/research/)
- **DeSci Ecosystem**: [awesome-desci](https://github.com/DeSciWorldDAO/awesome-desci)

---

**Status**: Alpha - Active Development (v0.1.0)
**Contact**: dev@mycelix.org (coming soon)
**Website**: https://mycelix.org (coming soon)
