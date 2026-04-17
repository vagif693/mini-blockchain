# ⛓️ mini-blockchain

> A proof-of-concept blockchain engine built from scratch in Rust — featuring SHA-256 hashing, Proof of Work mining, and cryptographic tamper detection.

![Rust](https://img.shields.io/badge/Rust-1.75+-orange?style=flat-square&logo=rust)
![License](https://img.shields.io/badge/License-MIT-blue?style=flat-square)
![Status](https://img.shields.io/badge/Status-Active-brightgreen?style=flat-square)

---

## 📸 Demo

\`\`\`
🚀 Starting Mini Blockchain

⛏️  Mining block 0...
✅ Block mined! Nonce: 125  | Hash: 00c700c4d1f5...
⛏️  Mining block 1...
✅ Block mined! Nonce: 174  | Hash: 00dc2c7b60d7...

✅ Blockchain is valid!

🔧 Tampering with Block 1...
❌ Blockchain is INVALID! Tampering detected.
\`\`\`

---

## 🧠 How It Works

A blockchain is a linked list of blocks where each block contains a cryptographic hash of the one before it. Changing any block breaks all subsequent links — making fraud instantly detectable.

### Block Structure

\`\`\`rust
struct Block {
    index:         u64,     // Position in the chain
    timestamp:     String,  // RFC3339 creation time
    data:          String,  // Transaction payload
    previous_hash: String,  // Hash of the previous block
    hash:          String,  // SHA-256 fingerprint of this block
    nonce:         u64,     // Value found during mining
}
\`\`\`

### Proof of Work

To add a block, the miner must find a nonce such that:

\`\`\`
SHA256(index + timestamp + data + previous_hash + nonce)
    must start with "00..."  (configurable difficulty)
\`\`\`

### Tamper Detection

is_valid() checks two things for every block:
1. Hash integrity — recalculates and compares the stored hash
2. Chain linkage — verifies previous_hash matches the actual previous block

---

## 🛠️ Tech Stack

| Crate | Version | Purpose |
|---|---|---|
| sha2 | 0.10 | SHA-256 hashing |
| hex | 0.4 | Encode hash bytes as readable strings |
| serde + serde_json | 1.0 | Block serialization |
| chrono | 0.4 | RFC3339 timestamps |

---

## ▶️ Getting Started

**Prerequisites:** Rust 1.75+ (https://rustup.rs)

\`\`\`bash
git clone https://github.com/vagif693/mini-blockchain.git
cd mini-blockchain/mini-blockchain
cargo run
\`\`\`

---

## 🗺️ Roadmap

- [x] SHA-256 block hashing
- [x] Proof of Work mining with configurable difficulty
- [x] Cryptographic chain validation
- [x] Tamper detection demo
- [ ] CLI interface with clap
- [ ] Persist chain to JSON file
- [ ] Wallet addresses and digital signatures

---

## 💡 Key Rust Concepts Demonstrated

- **Ownership & borrowing** — Rust's core memory model used throughout
- **Structs and impl blocks** — clean OOP-style design in Rust
- **Cryptographic hashing** — real SHA-256 via the sha2 crate
- **Iterators** — chain validation using Rust iterators

---

## 📄 License

MIT
