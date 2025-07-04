# Solana Starter

A comprehensive Solana development starter kit with TypeScript and Rust implementations for common blockchain operations including NFT minting, SPL token management, and vault operations.

## 🚀 Features

- **NFT Operations**: Mint, metadata management, and image upload
- **SPL Token Management**: Initialize, mint, transfer, and manage token metadata
- **Vault System**: Deposit, withdraw, and manage NFTs and SPL tokens
- **Wallet Tools**: Key generation, airdrops, and wallet utilities
- **Multi-language Support**: TypeScript and Rust implementations

## 📁 Project Structure

```
Solana-Starter/
├── ts/                          # TypeScript implementation
│   ├── cluster1/                # Main cluster operations
│   │   ├── nft_*.ts            # NFT-related operations
│   │   ├── spl_*.ts            # SPL token operations
│   │   ├── vault_*.ts          # Vault management
│   │   └── programs/           # Custom programs
│   ├── tools/                   # Utility scripts
│   ├── prereqs/                 # Prerequisites and setup
│   └── package.json
├── rs/                          # Rust implementation
│   ├── src/
│   │   ├── prereqs.rs          # Rust prerequisites
│   │   ├── cluster1.rs         # Cluster operations
│   │   └── programs/           # Custom Rust programs
│   └── Cargo.toml
└── README.md
```

## 🛠️ Prerequisites

### Required Software
- Node.js (v18 or higher)
- Rust (latest stable)
- Solana CLI tools
- Yarn or npm

### Solana Setup
```bash
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Set to devnet
solana config set --url devnet

# Create a new wallet (optional)
solana-keygen new
```

## 📦 Installation

### TypeScript Setup
```bash
cd ts
yarn install
```

### Rust Setup
```bash
cd rs
cargo build
```

## 🎯 Quick Start

### 1. Wallet Setup
```bash
# Generate a new wallet
yarn keygen

# Get airdrop (devnet)
yarn airdrop
```

### 2. SPL Token Operations
```bash
# Initialize SPL token
yarn spl_init

# Mint tokens
yarn spl_mint

# Transfer tokens
yarn spl_transfer
```

### 3. NFT Operations
```bash
# Upload NFT image
yarn nft_image

# Create metadata
yarn nft_metadata

# Mint NFT
yarn nft_mint
```

### 4. Vault Operations
```bash
# Initialize vault
yarn vault_init

# Deposit assets
yarn vault_deposit
yarn vault_deposit_spl
yarn vault_deposit_nft

# Withdraw assets
yarn vault_withdraw
yarn vault_withdraw_spl
yarn vault_withdraw_nft
```

## 🔧 Available Scripts

### Prerequisites
- `yarn keygen` - Generate new wallet keypair
- `yarn airdrop` - Request SOL airdrop
- `yarn transfer` - Transfer SOL between wallets
- `yarn enroll` - Enroll in devnet

### Tools
- `yarn airdrop_to_wallet` - Airdrop to specific wallet
- `yarn base58_to_wallet` - Convert base58 to wallet format
- `yarn wallet_to_base58` - Convert wallet to base58 format

### SPL Token Operations
- `yarn spl_init` - Initialize SPL token
- `yarn spl_mint` - Mint SPL tokens
- `yarn spl_transfer` - Transfer SPL tokens
- `yarn spl_metadata` - Manage token metadata

### NFT Operations
- `yarn nft_image` - Upload NFT image to IPFS
- `yarn nft_metadata` - Create NFT metadata
- `yarn nft_mint` - Mint NFT with metadata

### Vault Operations
- `yarn vault_init` - Initialize vault
- `yarn vault_deposit` - Deposit SOL to vault
- `yarn vault_withdraw` - Withdraw SOL from vault
- `yarn vault_deposit_spl` - Deposit SPL tokens to vault
- `yarn vault_withdraw_spl` - Withdraw SPL tokens from vault
- `yarn vault_deposit_nft` - Deposit NFT to vault
- `yarn vault_withdraw_nft` - Withdraw NFT from vault
- `yarn vault_close` - Close vault

## 🔑 Configuration

### Environment Setup
The project uses a wallet file (`Turbin3-wallet.json`) for authentication. Make sure to:
1. Generate your wallet using `yarn keygen`
2. Fund your wallet using `yarn airdrop`
3. Update the wallet path in scripts if needed

### RPC Endpoint
Default RPC endpoint is set to Solana devnet:
```
https://api.devnet.solana.com
```

## 📚 Dependencies

### TypeScript Dependencies
- `@metaplex-foundation/umi` - Metaplex UMI framework
- `@solana/web3.js` - Solana Web3.js library
- `@solana/spl-token` - SPL token program
- `@coral-xyz/anchor` - Anchor framework
- `bs58` - Base58 encoding

### Rust Dependencies
- `solana-sdk` - Solana SDK
- `solana-client` - Solana client library
- `solana-program` - Solana program library
- `borsh` - Serialization library

## 🚨 Important Notes

1. **Network**: All operations are configured for Solana devnet by default
2. **Wallet Security**: Keep your wallet keypair secure and never commit it to version control
3. **Rate Limits**: Be mindful of RPC rate limits when running multiple operations
4. **Gas Fees**: Ensure your wallet has sufficient SOL for transaction fees

## 🔍 Troubleshooting

### Common Issues
1. **Insufficient SOL**: Use `yarn airdrop` to get devnet SOL
2. **RPC Errors**: Check your internet connection and RPC endpoint
3. **Wallet Issues**: Regenerate wallet using `yarn keygen`

### Debug Mode
Enable debug logging by setting environment variables:
```bash
export DEBUG=solana:*
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request


## 🔗 Resources

- [Solana Documentation](https://docs.solana.com/)
- [Metaplex Documentation](https://docs.metaplex.com/)
- [Anchor Documentation](https://www.anchor-lang.com/)
- [Solana Devnet Explorer](https://explorer.solana.com/?cluster=devnet) 