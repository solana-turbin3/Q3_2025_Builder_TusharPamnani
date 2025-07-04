# Turbin3 — Prerequisites & Starter

This repository contains my submissions and practice code for the **Turbin3**.

![Solana](https://img.shields.io/badge/Solana-Devnet-3ECF8E?logo=solana\&logoColor=white)
![Anchor](https://img.shields.io/badge/Anchor-Framework-blueviolet)

---

## 📁 Folder Structure

```
.
├── 01. Turbin3-TS-PreReq
├── 02. Turbin3-RS-PreReq
├── Solana-Starter
├── README.md
```

### 01. Turbin3-TS-PreReq

TypeScript-based prerequisite tasks for the bootcamp: <br />
✅ Created and funded dev & Turbin3 wallets <br />
✅ Transferred funds between wallets <br />
✅ Minted my **proof-of-completion NFT** <br />
✅ Stored my GitHub username on-chain <br />

Key files:

* `airdrop.ts` — Airdrop SOL to wallet
* `keygen.ts` — Generate wallets
* `enroll.ts` — Enroll in the program (on-chain)
* `transfer.ts` — Transfer SOL
* `test.ts`, `decodeWallet.ts` — Misc helpers
* `programs/Turbin3_prereq.ts` — Smart contract interaction

---

### 02. Turbin3-RS-PreReq

Rust-based prerequisite tasks for the bootcamp: <br />
✅ Created and funded dev & Turbin3 wallets <br />
✅ Transferred funds between wallets <br />
✅ Minted my **proof-of-completion NFT** <br />
✅ Stored my GitHub username on-chain <br />

Key functions:

| File                   | Purpose                                               |
| ---------------------- | ----------------------------------------------------- |
| `keygen()`             | Generate a new wallet and save the private key        |
| `airdrop()`            | Airdrop 2 SOL on devnet                               |
| `transfer_sol()`       | Transfer 0.1 SOL to the registered Turbin3 wallet     |
| `drain_wallet()`       | Transfer all remaining funds to the Turbin3 wallet    |
| `submit_rs()`          | Call the on-chain `submit_rs` instruction to mint NFT |
| `print_pda_and_mint()` | Debug helper to print derived PDA and signer pubkey   |

---

### Solana-Starter

This folder contains the starter code provided in the bootcamp for initial classes & exploration.

Key topics & files:

* NFT Minting:

  * `nft_image.ts` — Upload & store NFT image
  * `nft_metadata.ts` — Create & upload NFT metadata
  * `nft_mint.ts` — Mint an NFT on devnet

* SPL Tokens:

  * `spl_init.ts` — Initialize an SPL token mint
  * `spl_metadata.ts` — Attach metadata to SPL token
  * `spl_mint.ts` — Mint SPL tokens

Additional utilities & vault interactions are also included under `cluster1`, `prereqs`, `programs`, and `tools`.

---

## 🛠️ Getting Started

### Install dependencies

```bash
cd 01.\ Turbin3-TS-PreReq
yarn install
```

or for Solana-Starter:

```bash
cd Solana-Starter/ts
yarn install
```

### Run a script

```bash
yarn <script>
```

> Replace `<script>` with the desired file name (e.g., `nft_mint`).

---

## 📜 Notes

* All tasks were run on **Solana Devnet**.
* Rust contracts use the Anchor framework.
* NFT & SPL examples are self-explanatory by filename and demonstrate the basic Solana flows.

---

## 🔗 Useful Links

* [Solana Developer Docs](https://docs.solana.com/)
* [Anchor Framework Docs](https://book.anchor-lang.com/)
* [Metaplex NFT Standard](https://docs.metaplex.com/)
* [SPL Token Docs](https://spl.solana.com/token)

