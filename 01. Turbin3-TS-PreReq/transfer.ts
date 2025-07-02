import {
    Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, Transaction, SystemProgram, sendAndConfirmTransaction
  } from "@solana/web3.js";
  import wallet from "./dev-wallet.json";
  
  const from = Keypair.fromSecretKey(new Uint8Array(wallet));
  const to = new PublicKey("8abFkZ8kazx33ieEAvXPRxTANmYshhxN72CdLRsSonaw");
  const connection = new Connection("https://api.devnet.solana.com");
  
  (async () => {
    try {
      const transaction = new Transaction().add(SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: LAMPORTS_PER_SOL / 100,
      }));
      transaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
      transaction.feePayer = from.publicKey;
      const signature = await sendAndConfirmTransaction(connection, transaction, [from]);
      console.log(`TX: https://explorer.solana.com/tx/${signature}?cluster=devnet`);
    } catch (e) {
      console.error(e);
    }
  })();
  