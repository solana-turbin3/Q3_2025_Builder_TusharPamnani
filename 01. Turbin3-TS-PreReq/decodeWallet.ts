import bs58 from "bs58";
import fs from "fs";

// Your base58 private key string
const base58SecretKey = "your_private_key";

// Decode to Uint8Array
const secretKey = bs58.decode(base58SecretKey);

// Save to Turbin3-wallet.json
fs.writeFileSync("Turbin3-wallet.json", JSON.stringify(Array.from(secretKey)));

console.log("✅ Turbin3-wallet.json created successfully!");
