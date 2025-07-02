# ✅ Turbin3 Rust Prerequisite

This repository documents my Rust-based completion of the [Turbin3](https://turbin3.xyz) Builders Cohort prerequisites.

![Turbin3 Prereqs](https://img.shields.io/badge/Turbin3%20Verified-%F0%9F%92%9A%20Rust%20%26%20TS-green?style=for-the-badge&logo=solana&logoColor=white)


I’ve successfully:
- Created and managed Solana wallets in Rust
- Airdropped SOL on devnet
- Transferred and drained funds using raw transactions
- Derived PDAs and minted an NFT using the `submit_rs` instruction
- Interacted with the Anchor program without using Anchor CLI or TypeScript

---

## ⚙️ Rust Project Setup

```bash
cargo init --lib
````

### Dependencies (`Cargo.toml`)

```toml
[dependencies]
solana-sdk = "1.15.2"
solana-client = "1.15.2"
solana-program = "1.15.2"
bs58 = "0.4"
```

---

## 📂 File Overview

| File                   | Purpose                                               |
| ---------------------- | ----------------------------------------------------- |
| `keygen()`             | Generate a new wallet and save the private key        |
| `airdrop()`            | Airdrop 2 SOL on devnet                               |
| `transfer_sol()`       | Transfer 0.1 SOL to the registered Turbin3 wallet     |
| `drain_wallet()`       | Transfer all remaining funds to the Turbin3 wallet    |
| `submit_rs()`          | Call the on-chain `submit_rs` instruction to mint NFT |
| `print_pda_and_mint()` | Debug helper to print derived PDA and signer pubkey   |

---

## 🔗 Transaction Links

| Action                | Tx Hash               | Explorer                                                                    |
| --------------------- | --------------------- | --------------------------------------------------------------------------- |
| Airdrop               | `3oEeYXgusadHkTS7a8miZk3yypVBB2ZSfooSCBkwhq4peUZ3Y9ophxEFD2HyJCLHxwTvdGJnKeK9AXcLEHik4wYY` | [View on Solscan](https://solscan.io/tx/3oEeYXgusadHkTS7a8miZk3yypVBB2ZSfooSCBkwhq4peUZ3Y9ophxEFD2HyJCLHxwTvdGJnKeK9AXcLEHik4wYY?cluster=devnet) |
| Transfer 0.001 SOL      | `5cZKBnyYA2t4JcqAF8WGucgetccVBhJmWmfL5R5pvTH1U996x6aPwBnsxduDuvESw4WTKnDK5P4ZxnjfGunQQ45L` | [View on Solscan](https://solscan.io/tx/5cZKBnyYA2t4JcqAF8WGucgetccVBhJmWmfL5R5pvTH1U996x6aPwBnsxduDuvESw4WTKnDK5P4ZxnjfGunQQ45L?cluster=devnet) |
| Empty Wallet          | `49RaqjCZMW2716p3dS3wWmeCFuwDoNniDjKdkuK9mkomHgW1zEKKY3Uog8mVnjMX2724brzTtLEF72KLu4cAmN9y` | [View on Solscan](https://solscan.io/tx/49RaqjCZMW2716p3dS3wWmeCFuwDoNniDjKdkuK9mkomHgW1zEKKY3Uog8mVnjMX2724brzTtLEF72KLu4cAmN9y?cluster=devnet) |
| Mint NFT (submit\_rs) | `3b4AoyzqTRvC4MMH1Lvk77mZV9ekPxrceSwdbaxAZhQAFFuZek5sdtw7DrVmFJM3NokpSrJL2U3VSKcPQf6fESEB` | [View on Solscan](https://solscan.io/tx/3b4AoyzqTRvC4MMH1Lvk77mZV9ekPxrceSwdbaxAZhQAFFuZek5sdtw7DrVmFJM3NokpSrJL2U3VSKcPQf6fESEB?cluster=devnet) |

---

## 📇 On-Chain Proof

| Field           | Value                                          |
| --------------- | ---------------------------------------------- |
| Turbin3 Wallet  | `8abFkZ8kazx33ieEAvXPRxTANmYshhxN72CdLRsSonaw` |
| PDA Address     | `5GQeetHTQJ6vAPVVaMRvSY7vuFufSyTBDQT8ryG4UQmB` |
| Mint Address    | `8QEdGNnYKF7ZJyVyNFcVsGDV2iThteoKkDm9gHMU83kp` |
| GitHub Username | `tusharpamnani`                                |
| Rust Submit ✅   | Yes                                            |

---

## 🧠 Learning Highlights

* Manually built Solana instructions without Anchor CLI
* Worked with PDAs, Keypairs, and raw base58 encoding
* Understood how Anchor instructions are identified via discriminators
* Verified end-to-end that the Rust flow matches the TypeScript one

---

## 🙌 Shoutout

Huge thanks to the Turbin3 team and the community for designing this low-level deep dive into Solana and Anchor. Looking forward to Week 1!

---
