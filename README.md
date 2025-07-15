# Turbin3 — Prerequisites & Starter

This repository contains my submissions and practice code for the **Turbin3**.

![Solana](https://img.shields.io/badge/Solana-Devnet-3ECF8E?logo=solana&logoColor=white)
![Anchor](https://img.shields.io/badge/Anchor-Framework-blueviolet)

---

## 📁 Folder Structure

```
.
├── 01. Turbin3-TS-PreReq
├── 02. Turbin3-RS-PreReq
├── 03. Solana-Starter
├── 04. Vault
├── 05. Escrow
├── 06. AMM
├── 07. escrow_mike
├── nft_staking
├── test-ledger
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

### 03. Solana-Starter

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

### 04. Vault

A Solana smart contract (Anchor) for secure SOL vault operations.  
**Features:**
- Users can initialize a personal vault (using PDAs for security).
- Deposit and withdraw SOL securely.
- Close the vault to reclaim rent and remaining funds.
- Built-in tests demonstrate the full lifecycle: initialize → deposit → withdraw → close.
- Uses Anchor’s account validation and CPI for secure fund transfers.

**Key Concepts:**
- **PDAs (Program Derived Addresses):** Used for both vault state and vault accounts, ensuring only the owner can access funds.
- **Account Structure:** Vault state stores bump seeds for address derivation and security.

**How to use:**
- Build and deploy with Anchor CLI.
- Run tests to see transaction flows and account balances.
- See the escrow project’s README for sample commands and test output.

---

### 05. Escrow

A complete Anchor-based escrow system for secure SPL token swaps.  
**Features:**
- Users can create, accept, and refund escrow offers for SPL tokens.
- Two main programs: a simple SOL vault and a token escrow system.
- Includes a React frontend for interacting with the escrow contract.
- Comprehensive test suite for both vault and escrow flows.

**Key Concepts:**
- **Escrow Program:** Enables atomic swaps between two parties, ensuring both sides fulfill their obligations before tokens are released.
- **Vault Program:** Lets users securely store and manage SOL.
- **Frontend:** React app for user-friendly interaction with the escrow contract.

**How to use:**
- Install dependencies and build with Anchor.
- Deploy to localnet/devnet.
- Use the frontend or CLI to create and accept escrow offers.
- Run tests to verify all flows.

---

### 06. AMM

Automated Market Maker (AMM) built with Anchor and a Next.js frontend.  
**Features:**
- On-chain liquidity pools for token swaps.
- Instructions for swap, deposit, withdraw, and pool initialization.
- Uses the constant product formula for pricing (x*y=k).
- State management for pool reserves and user positions.
- Next.js frontend for interacting with the AMM on localhost.

**Key Concepts:**
- **Constant Product Curve:** Ensures fair pricing and liquidity for swaps.
- **Anchor Program:** Handles all pool logic, state, and validation.
- **Frontend:** Next.js app for easy pool interaction and visualization.

**How to use:**
- Build and deploy the Anchor program.
- Start the Next.js frontend (`npm run dev` in `amm-frontend`).
- Interact with pools, provide liquidity, and swap tokens via the UI.

---

### 07. escrow_mike

A modern, modular Anchor escrow implementation for teaching and real-world use.  
**Features:**
- Clean, warning-free builds with the latest Solana/Anchor/Rust.
- Modular handler structure: separate files for make_offer, take_offer, refund_offer, and shared logic.
- Comprehensive tests using Node.js and Anchor CLI.
- Designed for clarity and learning, with a full animated walkthrough available.

**Key Concepts:**
- **Handlers:** Each escrow action is implemented in its own handler for maintainability.
- **Offer State:** Tracks escrow offers and their status.
- **Testing:** Easy to run and extend, with clear output and CI integration.

**How to use:**
- Clone the repo and install dependencies.
- Run tests with `anchor test` (using the recommended toolchain).
- Deploy and interact with the program as described in the project README.

---

### nft_staking

NFT staking program with configuration, user, and stake account management. Includes instructions for initializing config, staking, and user setup. Built with Anchor.

---

### test-ledger

Local Solana test validator and ledger files for development and testing.

---

## 🛠️ Getting Started

### Install dependencies

```bash
cd 01.\ Turbin3-TS-PreReq
yarn install
```

or for Solana-Starter:

```bash
cd 03. Solana-Starter/ts
yarn install
```

### 04. Vault

```bash
cd 04.\ Vault
# Install dependencies
yarn install
# Build and deploy the program
anchor build
anchor deploy
# Run tests
yarn test
```

### 05. Escrow

```bash
cd 05.\ Escrow
# Install dependencies for backend and frontend
yarn install
cd escrow-frontend
yarn install
# Build and deploy the programs
cd ..
anchor build
anchor deploy
# Run tests
yarn test
# Start the frontend (in escrow-frontend)
yarn dev
```

### 06. AMM

```bash
cd 06.\ AMM
# Install dependencies for backend and frontend
yarn install
cd amm-frontend
yarn install
# Build and deploy the Anchor program
cd ..
anchor build
anchor deploy
# Run tests
yarn test
# Start the frontend (in amm-frontend)
yarn dev
```

### 07. escrow_mike

```bash
cd 07.\ escrow_mike
# Install dependencies
npm install
# Build and deploy the program
anchor build
anchor deploy
# Run tests
anchor test
```

### Run a script

```bash
yarn <script>
```

> Replace `<script>` with the desired file name (e.g., `nft_mint`). 

---

## 🔗 Useful Links

* [Solana Developer Docs](https://docs.solana.com/)
* [Anchor Framework Docs](https://book.anchor-lang.com/)
* [Metaplex NFT Standard](https://docs.metaplex.com/)
* [SPL Token Docs](https://spl.solana.com/token)

