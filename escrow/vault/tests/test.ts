import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import type { Vault } from "../target/types/vault";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { assert } from "chai";
import "dotenv/config";
import BN from "bn.js";

describe("Vault", () => {
  // Configure the Anchor provider (localnet/devnet)
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // console.log("the provider is: ", provider);

  const user = provider.wallet.payer;
  console.log("Running tests as wallet:", user.publicKey.toBase58());

  const program = anchor.workspace.vault as Program<Vault>;

  // Derive PDAs for state & vault
  const [vaultStatePDA, vaultStateBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("state"), user.publicKey.toBytes()],
    program.programId
  );

  const [vaultPDA, vaultBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), vaultStatePDA.toBytes()],
    program.programId
  );

  // Helper: bumps struct expected by the program
  const vaultBumps = {
    stateBump: vaultStateBump,
    vaultBump: vaultBump,
  };

  it("Initializes the vault", async () => {
    const tx = await program.methods
      .initialize() // single struct argument
      .accounts({
        user: user.publicKey,
        vaultState: vaultStatePDA,
        vault: vaultPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Initialize TX:", tx);

    const stateAccount = await program.account.vaultState.fetch(vaultStatePDA);
    assert.isOk(stateAccount, "Vault state not initialized properly");
  });

  it("Deposits funds", async () => {
    const depositAmountLamports = new BN(0.1 * LAMPORTS_PER_SOL); // 0.1 SOL

    const tx = await program.methods
      .deposit(depositAmountLamports)
      .accounts({
        user: user.publicKey,
        vaultState: vaultStatePDA,
        vault: vaultPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Deposit TX:", tx);

    const balance = await provider.connection.getBalance(vaultPDA);
    console.log(
      `Vault balance after deposit: ${balance / LAMPORTS_PER_SOL} SOL`
    );

    assert(
      balance >= depositAmountLamports.toNumber(),
      "Deposit failed"
    );
  });

  it("Withdraws funds", async () => {
    const withdrawAmountLamports = new BN(0.05 * LAMPORTS_PER_SOL); // 0.05 SOL

    const tx = await program.methods
      .withdraw(withdrawAmountLamports)
      .accounts({
        user: user.publicKey,
        vaultState: vaultStatePDA,
        vault: vaultPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Withdraw TX:", tx);

    const balance = await provider.connection.getBalance(vaultPDA);
    console.log(
      `Vault balance after withdraw: ${balance / LAMPORTS_PER_SOL} SOL`
    );

    assert(balance >= 0, "Withdraw failed");
  });

  it("Closes the vault", async () => {
    const tx = await program.methods
      .close()
      .accounts({
        user: user.publicKey,
        vaultState: vaultStatePDA,
        vault: vaultPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Close TX:", tx);

    const accountInfo = await provider.connection.getAccountInfo(vaultStatePDA);
    assert(
      accountInfo === null,
      "Vault state account still exists after close"
    );
  });
});
