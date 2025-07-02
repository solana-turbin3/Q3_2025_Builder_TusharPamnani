import {
    Connection,
    Keypair,
    PublicKey,
    SystemProgram,
  } from "@solana/web3.js";
  import { Program, AnchorProvider, Wallet } from "@coral-xyz/anchor";
  import { IDL, TurbinPreReq } from "./programs/Turbin3_prereq";
  import walletData from "./Turbin3-wallet.json";
  
  const connection = new Connection("https://api.devnet.solana.com");

  const keypair = Keypair.fromSecretKey(new Uint8Array(walletData));
  const provider = new AnchorProvider(connection, new Wallet(keypair), {
    commitment: "confirmed",
  });


  const program = new Program<typeof IDL>(
    IDL,
    provider
  );
  
  // Create PDA
  const accountSeeds = [Buffer.from("prereqs"), keypair.publicKey.toBuffer()];
  const [accountKey, _bump] = PublicKey.findProgramAddressSync(accountSeeds, program.programId);
  
  // Constants
  const githubUsername = "tusharpamnani"; // CHANGE THIS
  const MPL_CORE_PROGRAM_ID = new PublicKey("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");
  const mintCollection = new PublicKey("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2");
  const mintTs = Keypair.generate();
  
  // Run `initialize` transaction
//   (async () => {
//     try {
//       const tx = await program.methods
//         .initialize(githubUsername)
//         .accounts({
//           user: keypair.publicKey,
//           account: accountKey,
//           systemProgram: SystemProgram.programId,
//         })
//         .signers([keypair])
//         .rpc();
  
//       console.log(`✅ initialize() TX: https://explorer.solana.com/tx/${tx}?cluster=devnet`);
//     } catch (err) {
//       console.error("❌ initialize() failed:", err);
//     }
//   })();
  

  // Run `submitTs` transaction
  (async () => {
    try {
      const [authorityPDA] = PublicKey.findProgramAddressSync(
        [Buffer.from("collection"), mintCollection.toBuffer()],
        program.programId
      );
  
      const tx = await program.methods
        .submitTs()
        .accounts({
          user: keypair.publicKey,
          account: accountKey,
          mint: mintTs.publicKey,
          collection: mintCollection,
          authority: authorityPDA,
          mplCoreProgram: MPL_CORE_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([keypair, mintTs])
        .rpc();
  
      console.log(`✅ submitTs() TX: https://explorer.solana.com/tx/${tx}?cluster=devnet`);
    } catch (err) {
      console.error("❌ submitTs() failed:", err);
    }
  })();
  