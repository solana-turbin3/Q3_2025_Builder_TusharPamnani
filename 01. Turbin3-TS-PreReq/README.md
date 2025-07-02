# ✅ Turbin3 TypeScript Prerequisite

This repository contains my completed tasks for the TypeScript prerequisite of [Turbin3](https://turbin3.xyz)'s on-chain learning journey.

I’ve successfully:
- Created and funded dev and Turbin3 wallets
- Transferred funds
- Interacted with the smart contract using Anchor
- Minted my proof-of-completion NFT
- Stored my GitHub username on-chain!

---

## 🛠️ Project Setup

```bash
yarn init -y
yarn add @solana/web3.js @coral-xyz/anchor bs58 prompt-sync
yarn add -D ts-node typescript @types/node
````

Update `package.json` scripts:

```json
"scripts": {
   "keygen": "ts-node ./keygen.ts",
    "airdrop": "ts-node ./airdrop.ts",
    "transfer": "ts-node ./transfer.ts",
    "enroll": "ts-node ./enroll.ts",
    "empty": "ts-node ./empty.ts",
    "decode" : "ts-node ./decodeWallet.ts",
    "test": "ts-node ./test.ts"
}
```

---

## 📂 File Overview

| File                         | Purpose                                                                 |
| ---------------------------- | ----------------------------------------------------------------------- |
| `keygen.ts`                  | Generates a new dev wallet (keypair) and outputs public/private keys    |
| `airdrop.ts`                 | Requests 2 SOL to your devnet wallet                                    |
| `transfer.ts`                | Sends 0.1 SOL from your dev wallet to Turbin3’s wallet                  |
| `empty.ts`                   | Transfers **remaining SOL** from dev wallet to Turbin3 wallet           |
| `enroll.ts`                  | Registers your GitHub via `initialize()` and mints NFT via `submitTs()` |
| `validate.ts`                | Fetches your on-chain `applicationAccount` to confirm GitHub + status   |
| `programs/Turbin3_prereq.ts` | IDL for interacting with the Turbin3 smart contract                     |
| `Turbin3-wallet.json`        | Your Turbin3 wallet (used to sign transactions)                         |
| `dev-wallet.json`            | Your devnet wallet (used to fund and transfer)                          |

---

## 🔗 Transaction Links

| Action              | Tx Hash | Explorer             |
| ------------------- | ------- | -------------------- |
| Airdrop             | `37LDuZfHXZGLZHhhT9dF3xHzfkKQZgkP8mCZdW832JVy8oWWAQRWfn3mkCa5VHCxd9mGqHSo7n8eZMAoz3WvcvCu`   | [View on Solscan](https://solscan.io/tx/37LDuZfHXZGLZHhhT9dF3xHzfkKQZgkP8mCZdW832JVy8oWWAQRWfn3mkCa5VHCxd9mGqHSo7n8eZMAoz3WvcvCu?cluster=devnet) |
| Transfer 0.1 SOL    | `4nbaXGnao39DWnpC7JPVbsNLFZo8YmSXnyNsFk9mnBfnYThSU2oAG9zPsxnB8RTAwmpZtvYNsWsU6ZGX3jJEZffc`   | [View on Solscan](https://solscan.io/tx/4nbaXGnao39DWnpC7JPVbsNLFZo8YmSXnyNsFk9mnBfnYThSU2oAG9zPsxnB8RTAwmpZtvYNsWsU6ZGX3jJEZffc?cluster=devnet) |
| Empty Wallet        | `4LBSjEHpZyLdXAiR8UrmrepBN7FJNTv8mmY51HvFStmmecHXAUne1pWZU8RCFyUkohJZEJQVcdUnECoWcjoUauC`   | [View on Solscan](https://solscan.io/tx/4LBSjEHpZyLdXAiR8UrmrepBN7FJNTv8mmY51HvFStmmecHXAUne1pWZU8RCFyUkohJZEJQVcdUnECoWcjoUauC?cluster=devnet) |
| Initialize (GitHub) | `3UVMKYtdrEGyRk1FTzDE6ccbs2PEnbFQPZ8DHXeUAcDd3NwpC2TMMjj2NBAdRwkCuNNiZmqDapTg6AxYNefijknr`   | [View on Solscan](https://solscan.io/tx/3UVMKYtdrEGyRk1FTzDE6ccbs2PEnbFQPZ8DHXeUAcDd3NwpC2TMMjj2NBAdRwkCuNNiZmqDapTg6AxYNefijknr?cluster=devnet) |
| Mint NFT (submitTs) | `46F2tWMe1GUkcajYCKLKThtjhANuugWK9AQrXwukiwyCK5df5WjVMTrsVMn64hdMKGv5p596GjsgbBZi98dKudwz`   | [View on Solscan](https://solscan.io/tx/46F2tWMe1GUkcajYCKLKThtjhANuugWK9AQrXwukiwyCK5df5WjVMTrsVMn64hdMKGv5p596GjsgbBZi98dKudwz?cluster=devnet) |

---

## 📇 On-Chain GitHub Verification

| Account             | Value                                          |
| ------------------- | ---------------------------------------------- |
| Turbin3 Wallet      | `8abFkZ8kazx33ieEAvXPRxTANmYshhxN72CdLRsSonaw` |
| PDA Address         | `5GQeetHTQJ6vAPVVaMRvSY7vuFufSyTBDQT8ryG4UQmB` |
| GitHub              | `tusharpamnani`                                |
| NFT Minted          | ✅ Yes                                          |
| TypeScript Complete | ✅ Yes                                          |
| Rust Complete       | ❌ Not yet                                      |

---

## 🙌 Shoutout

Thanks to the Turbin3 team and the community for building an amazing on-chain learning experience!

---
