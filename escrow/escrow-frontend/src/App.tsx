import React, { useMemo, useState } from "react";
import { Connection, PublicKey, SystemProgram, Commitment } from "@solana/web3.js";
import { useWallet, WalletProvider, ConnectionProvider } from "@solana/wallet-adapter-react";
import { WalletModalProvider, WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { PhantomWalletAdapter } from "@solana/wallet-adapter-phantom";
import { AnchorProvider, Program, BN } from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID, getAssociatedTokenAddress } from "@solana/spl-token";
import { Copy, Shuffle, Shield, ArrowRight, Check, X, Info } from 'lucide-react';

import idl from "./idl/escrow.json";
import { Buffer } from "buffer";
import "./App.css";
if (window.Buffer === undefined) {
  window.Buffer = Buffer;
}
require("@solana/wallet-adapter-react-ui/styles.css");

const programID = new PublicKey(idl.address);
const network = "https://api.devnet.solana.com"; // devnet endpoint
const commitment: Commitment = "processed";

function EscrowApp() {
  const wallet = useWallet();
  const [mintAInput, setMintAInput] = useState<string>("");
  const [mintBInput, setMintBInput] = useState<string>("");
  const [depositInput, setDepositInput] = useState<string>("");
  const [receiveInput, setReceiveInput] = useState<string>("");
  const [seedInput, setSeedInput] = useState<string>(() => (Math.floor(Math.random() * 1e9)).toString());
  const [escrowInfo, setEscrowInfo] = useState<any>(null);
  const [isCreating, setIsCreating] = useState(false);
  const [status, setStatus] = useState('');

  const connection = useMemo(() => new Connection(network, { commitment }), []);
  const provider = useMemo(
    () => new AnchorProvider(connection, wallet as any, { preflightCommitment: commitment }),
    [connection, wallet]
  );
  const program = useMemo(
    () => new Program(idl as any, provider),
    [provider]
  );

  const StatusIcon = ({ status }: { status: string }) => {
    if (status.startsWith('Error')) return <X className="w-5 h-5 text-red-500" />;
    if (status.startsWith('Escrow created')) return <Check className="w-5 h-5 text-green-500" />;
    return <Info className="w-5 h-5 text-blue-500" />;
  };

  const createEscrow = async () => {
    setStatus("Creating escrow...");
    setEscrowInfo(null);
    try {
      if (!wallet.publicKey) {
        setStatus("Wallet not connected");
        return;
      }
      if (!mintAInput || !mintBInput || !depositInput || !receiveInput || !seedInput) {
        setStatus("Please enter all fields.");
        return;
      }
      const mintA = new PublicKey(mintAInput);
      const mintB = new PublicKey(mintBInput);
      const seed = new BN(seedInput);
      const deposit = new BN(depositInput);
      const receive = new BN(receiveInput);
      // Derive escrow PDA
      const [escrowPda] = await PublicKey.findProgramAddress(
        [
          Buffer.from("escrow"),
          wallet.publicKey.toBuffer(),
          seed.toArrayLike(Buffer, "le", 8)
        ],
        program.programId
      );
      // Derive vault ATA (do NOT create it, let the program do it)
      const vaultAta = await getAssociatedTokenAddress(
        mintA,
        escrowPda,
        true // allowOwnerOffCurve
      );
      // Derive maker's ATA for mintA
      const makerAtaA = await getAssociatedTokenAddress(
        mintA,
        wallet.publicKey
      );
      console.log('maker:', wallet.publicKey.toBase58());
      console.log('mintA:', mintA.toBase58());
      console.log('makerAtaA:', makerAtaA.toBase58());
      // Call the make instruction
      const tx = await program.methods
        .make(seed, receive)
        .accounts({
          maker: wallet.publicKey,
          mintA,
          mintB,
          makerAtaA,
          escrow: escrowPda,
          vault: vaultAta,
          associatedTokenProgram: new PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .rpc();
      setStatus(`Escrow created! Tx: ${tx}`);
      // Fetch and display escrow account data
      const escrowAccount = await (program.account as any).escrow.fetch(escrowPda);
      setEscrowInfo(escrowAccount);
    } catch (e: any) {
      console.error("Full error object:", e);
      let errMsg = e.message || JSON.stringify(e);
      if (e.logs) errMsg += "\n" + e.logs.join("\n");
      setStatus("Error: " + errMsg);
    }
  };

  const randomizeSeed = () => {
    setSeedInput(Math.floor(Math.random() * 1e9).toString());
  };


  return (
    <div className="app-bg">
      <div className="main-card">
        {/* Header */}
        <div className="header">
          <div className="header-left">
            <div className="header-icon">
              <Shield />
            </div>
            <div>
              <h1 className="header-title">Solana Escrow</h1>
              <p className="header-desc">Secure token swaps on Solana</p>
            </div>
          </div>
          <div className="wallet-btn">
            <WalletMultiButton />
          </div>
        </div>

        {/* Card Header */}
        <div className="card-header">
          <h2 className="card-title">Create New Escrow</h2>
          <p className="card-desc">Set up a secure token swap with another party</p>
        </div>

        {/* Helper Info */}
        <div className="helper-info">
          <div className="helper-info-icon">
            <Info />
          </div>
          <div className="helper-info-text">
            <p className="helper-info-title">Quick Guide:</p>
            <ul className="helper-info-list">
              <li><strong>Mint A:</strong> Token you're offering</li>
              <li><strong>Mint B:</strong> Token you want to receive</li>
              <li><strong>Amounts:</strong> Use base units (e.g., 1000000000 for 1 token with 9 decimals)</li>
              <li><strong>Seed:</strong> Unique identifier for this escrow</li>
            </ul>
          </div>
        </div>

        {/* Form */}
        <div className="form-section">
          {/* Token A */}
          <div className="form-group">
            <label className="form-label">
              <span>Mint A Address</span>
              <span className="form-label-desc">(Token you're offering)</span>
            </label>
            <input
              className="form-input"
              value={mintAInput}
              onChange={e => setMintAInput(e.target.value)}
              placeholder="Enter Mint A address..."
            />
          </div>

          {/* Arrow Separator */}
          <div className="arrow-separator">
            <ArrowRight />
          </div>

          {/* Token B */}
          <div className="form-group">
            <label className="form-label">
              <span>Mint B Address</span>
              <span className="form-label-desc">(Token you want to receive)</span>
            </label>
            <input
              className="form-input"
              value={mintBInput}
              onChange={e => setMintBInput(e.target.value)}
              placeholder="Enter Mint B address..."
            />
          </div>

          {/* Amounts */}
          <div className="form-row">
            <div className="form-group">
              <label className="form-label">Amount to Deposit</label>
              <input
                className="form-input"
                value={depositInput}
                onChange={e => setDepositInput(e.target.value)}
                placeholder="e.g., 1000000000"
              />
            </div>
            <div className="form-group">
              <label className="form-label">Amount to Receive</label>
              <input
                className="form-input"
                value={receiveInput}
                onChange={e => setReceiveInput(e.target.value)}
                placeholder="e.g., 1000000000"
              />
            </div>
          </div>

          {/* Seed */}
          <div className="form-group form-seed-group">
            <label className="form-label">Seed</label>
            <div className="form-seed-row">
              <input
                className="form-input"
                value={seedInput}
                onChange={e => setSeedInput(e.target.value)}
                placeholder="Enter unique seed..."
              />
              <button
                onClick={randomizeSeed}
                className="form-seed-btn"
              >
                <Shuffle />
                <span className="form-seed-btn-text">Random</span>
              </button>
            </div>
          </div>

          {/* Create Button */}
          <button
            onClick={createEscrow}
            disabled={!wallet.connected || isCreating}
            className="create-btn"
          >
            {isCreating ? (
              <div className="create-btn-loading">
                <div className="create-btn-spinner"></div>
                <span>Creating Escrow...</span>
              </div>
            ) : (
              'Create Escrow'
            )}
          </button>
        </div>

        {/* Status Message */}
        {status && (
          <div className="status-message-container">
            <div className={`status-message${status.startsWith('Error') ? ' error' : status.startsWith('Escrow created') ? ' success' : ''}`}>
              <StatusIcon status={status} />
              <span className="status-message-text">{status}</span>
            </div>
          </div>
        )}

        {/* Escrow Info */}
        {escrowInfo && (
          <div className="escrow-info-container">
            <div className="escrow-info-card">
              <div className="escrow-info-header">
                <h3 className="escrow-info-title">Escrow Created</h3>
                <button className="escrow-info-copy-btn">
                  <Copy />
                </button>
              </div>
              <div className="escrow-info-list">
                <div className="escrow-info-row">
                  <span className="escrow-info-label">Escrow Account:</span>
                  <span className="escrow-info-value">{escrowInfo.escrowAccount}</span>
                </div>
                <div className="escrow-info-row">
                  <span className="escrow-info-label">Maker:</span>
                  <span className="escrow-info-value">{escrowInfo.maker}</span>
                </div>
                <div className="escrow-info-row">
                  <span className="escrow-info-label">Seed:</span>
                  <span className="escrow-info-value">{escrowInfo.seed}</span>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Footer */}
        <div className="footer">
          <p>Powered by Solana • Secure • Decentralized</p>
        </div>
      </div>
    </div>
  );
}

export default function App() {
  const wallets = useMemo(() => [new PhantomWalletAdapter()], []);
  return (
    <ConnectionProvider endpoint={network}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>
          <EscrowApp />
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
} 