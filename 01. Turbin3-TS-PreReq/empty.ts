import {
    Connection,
    Keypair,
    PublicKey,
    SystemProgram,
    Transaction,
    sendAndConfirmTransaction
  } from "@solana/web3.js";
  
  import wallet from "./dev-wallet.json";
  
  // Replace this with your Turbin3 address
  const turbin3Address = new PublicKey("8abFkZ8kazx33ieEAvXPRxTANmYshhxN72CdLRsSonaw");
  const from = Keypair.fromSecretKey(new Uint8Array(wallet));
  const connection = new Connection("https://api.devnet.solana.com");
  
  (async () => {
    try {
      // Get balance of your dev wallet
      const balance = await connection.getBalance(from.publicKey);
  
      // Create a dummy transaction to calculate the fee
      const tempTx = new Transaction().add(
        SystemProgram.transfer({
          fromPubkey: from.publicKey,
          toPubkey: turbin3Address,
          lamports: balance
        })
      );
  
      tempTx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
      tempTx.feePayer = from.publicKey;
  
      const fee = (await connection.getFeeForMessage(tempTx.compileMessage(), "confirmed")).value || 0;
  
      const actualTransferLamports = balance - fee;
  
      if (actualTransferLamports <= 0) {
        throw new Error("Not enough balance to cover transaction fee");
      }
  
      const finalTx = new Transaction().add(
        SystemProgram.transfer({
          fromPubkey: from.publicKey,
          toPubkey: turbin3Address,
          lamports: actualTransferLamports
        })
      );
  
      finalTx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
      finalTx.feePayer = from.publicKey;
  
      const signature = await sendAndConfirmTransaction(connection, finalTx, [from]);
      console.log(`✅ Success! View on Explorer:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`);
    } catch (e) {
      console.error("❌ Error:", e);
    }
  })();
  