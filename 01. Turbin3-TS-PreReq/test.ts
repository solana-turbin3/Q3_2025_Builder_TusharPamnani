import { PublicKey } from "@solana/web3.js";

const programId = new PublicKey("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM");
const turbin3Wallet = new PublicKey("8abFkZ8kazx33ieEAvXPRxTANmYshhxN72CdLRsSonaw");

const [pda] = PublicKey.findProgramAddressSync(
  [Buffer.from("prereqs"), turbin3Wallet.toBuffer()],
  programId
);

console.log("✅ Your PDA is:", pda.toBase58());
