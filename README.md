
# 🗳️ Soroban Voting Smart Contract

This is a simple decentralized voting smart contract built on the [Stellar Soroban](https://soroban.stellar.org) platform using Rust.

## 📦 Contract Overview

This smart contract allows users to:

- Add candidates for voting
- Vote for a candidate (one vote per address)
- View the list of candidates
- View vote count for a specific candidate

It uses Soroban’s storage system to persist candidate data and track voter activity.

---

## 🛠️ Project Structure

```

voting\_contract/
├── Cargo.toml         # Rust dependencies and build config
└── src/
└── lib.rs         # Main smart contract logic

````

---

## 🔧 Functions

### `initialize(env: Env)`
Initializes empty storage for candidates, votes, and voter status.

### `add_candidate(env: Env, candidate: Symbol)`
Adds a new candidate. Panics if the candidate already exists.

### `vote(env: Env, voter: Address, candidate: Symbol)`
Lets a user vote for a candidate. Only one vote per wallet address is allowed. Requires `require_auth`.

### `get_votes(env: Env, candidate: Symbol) -> u32`
Returns the current vote count for a specific candidate.

### `get_candidates(env: Env) -> Vec<Symbol>`
Returns the list of all registered candidates.

---

## 🧪 Build and Deploy

Make sure you have the [Soroban CLI](https://soroban.stellar.org/docs/getting-started/sdk/cli) and Rust toolchain installed.

### 🔨 Build Contract
```bash
soroban contract build
````

### 🛰️ Deploy to Futurenet (Testnet)

```bash
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/voting_contract.wasm
```

---

## ✅ Requirements

* Rust (>=1.70)
* Soroban SDK (`soroban-sdk = "20.1.0"`)
* Soroban CLI (latest)

---

## 📖 Resources

* [Soroban Docs](https://soroban.stellar.org/docs)
* [Soroban Examples](https://github.com/stellar/soroban-examples)
* [Stellar Developer Portal](https://developers.stellar.org)

---
