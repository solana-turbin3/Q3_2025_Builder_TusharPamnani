#[cfg(test)]
mod tests {
    use solana_sdk;

//     #[test]
//     fn keygen() {
//         use solana_sdk::signature::{ Keypair, Signer };
//         let kp = Keypair::new();
//         println!("You've generated a new Solana wallet: {}", kp.pubkey());
//         println!("To save your wallet, copy and paste the following into a JSON file:");
//         println!("{:?}", kp.to_bytes());
//     }

//     #[test]
//     fn airdrop() {
//         use solana_client::rpc_client::RpcClient;
//         use solana_sdk::signature::{ read_keypair_file, Signer }; // <-- FIXED: Added Signer trait

//         const RPC_URL: &str =
//             "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";

//         let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
//         let client = RpcClient::new(RPC_URL);

//         match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
//             Ok(sig) => {
//                 println!("✅ Airdrop successful! View TX at:");
//                 println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
//             }
//             Err(err) => {
//                 println!("❌ Airdrop failed: {}", err);
//             }
//         }
//     }


//     #[test]
//     fn base58_to_wallet() {
//         use std::io::{self, BufRead};
//         println!("Input your private key as a base58 string:");
//         let stdin = io::stdin();
//         let base58 = stdin.lock().lines().next().unwrap().unwrap();
//         let wallet = bs58::decode(base58).into_vec().unwrap();
//         println!("Your wallet file format is:\n{:?}", wallet);
//     }

//     #[test]
// fn wallet_to_base58() {
//     use std::io::{self, BufRead};
//     println!("Input your private key as a JSON byte array (e.g. [12,34,...]):");
//     let stdin = io::stdin();
//     let wallet = stdin
//         .lock()
//         .lines()
//         .next()
//         .unwrap()
//         .unwrap()
//         .trim_start_matches('[')
//         .trim_end_matches(']')
//         .split(',')
//         .map(|s| s.trim().parse::<u8>().unwrap())
//         .collect::<Vec<u8>>();
//     let base58 = bs58::encode(wallet).into_string();
//     println!("Your Base58-encoded private key is:\n{}", base58);
// }

// #[test]
// fn transfer_sol() {
//     use solana_client::rpc_client::RpcClient;
//     use solana_program::{pubkey::Pubkey, system_instruction::transfer};
//     use solana_sdk::{
//         signature::{read_keypair_file, Signer},
//         transaction::Transaction,
//     };
//     use std::str::FromStr;

//     const RPC_URL: &str = "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";

//     let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
//     let to_pubkey = Pubkey::from_str("8abFkZ8kazx33ieEAvXPRxTANmYshhxN72CdLRsSonaw").expect("Invalid pubkey");
//     let client = RpcClient::new(RPC_URL);
//     let recent_blockhash = client.get_latest_blockhash().expect("Failed to get blockhash");

//     let transaction = Transaction::new_signed_with_payer(
//         &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)], // 0.001 SOL
//         Some(&keypair.pubkey()),
//         &[&keypair],
//         recent_blockhash,
//     );

//     let signature = client
//         .send_and_confirm_transaction(&transaction)
//         .expect("Failed to send transaction");

//     println!(
//         "✅ Success! TX: https://explorer.solana.com/tx/{}?cluster=devnet",
//         signature
//     );
// }


// #[test]
// fn check_balance() {
//     use solana_client::rpc_client::RpcClient;
//     use solana_sdk::signature::{read_keypair_file, Signer};

//     const RPC_URL: &str = "https://api.devnet.solana.com"; // fallback RPC

//     let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
//     let client = RpcClient::new(RPC_URL);
//     let balance = client.get_balance(&keypair.pubkey()).unwrap();

//     println!("💰 Wallet balance: {} lamports ({} SOL)", balance, balance as f64 / 1_000_000_000.0);
// }

// #[test]
// fn drain_wallet() {
//     use solana_client::rpc_client::RpcClient;
//     use solana_program::{pubkey::Pubkey, system_instruction::transfer};
//     use solana_sdk::{
//         message::Message,
//         signature::{read_keypair_file, Signer},
//         transaction::Transaction,
//     };
//     use std::str::FromStr;

//     const RPC_URL: &str = "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";

//     let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet");
//     let to_pubkey = Pubkey::from_str("8abFkZ8kazx33ieEAvXPRxTANmYshhxN72CdLRsSonaw").expect("Invalid pubkey");
//     let client = RpcClient::new(RPC_URL);
//     let recent_blockhash = client.get_latest_blockhash().expect("Blockhash error");

//     let balance = client.get_balance(&signer.pubkey()).expect("Balance error");
//     let message = Message::new_with_blockhash(
//         &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
//         Some(&keypair.pubkey()),
//         &recent_blockhash,
//     );
//     let fee = client.get_fee_for_message(&message).expect("Fee error");

//     let tx = Transaction::new_signed_with_payer(
//         &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
//         Some(&keypair.pubkey()),
//         &[&keypair],
//         recent_blockhash,
//     );

//     let sig = client.send_and_confirm_transaction(&tx).expect("Transfer failed");
//     println!(
//         "✅ Wallet drained. TX: https://explorer.solana.com/tx/{}?cluster=devnet",
//         sig
//     );
// }

#[test]
fn submit_rs() {
    use solana_client::rpc_client::RpcClient;
    use solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    };
    use solana_sdk::{
        signature::{read_keypair_file, Keypair, Signer},
        transaction::Transaction,
    };
    use std::str::FromStr;

    const RPC_URL: &str = "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";

    // Load your Turbin3 wallet (the same one used in TypeScript)
    let signer = read_keypair_file("Turbin3-wallet.json").expect("Wallet not found");
    let signer_pubkey = signer.pubkey();

    // Define program addresses
    let turbin3_program = Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
    let collection = Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();
    let mpl_core_program = Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
    let system_program = system_program::id();

    // Derive the account PDA
    let (account_pda, _) = Pubkey::find_program_address(&[b"prereqs", signer_pubkey.as_ref()], &turbin3_program);

    // Derive authority PDA from collection
    let (authority_pda, _) = Pubkey::find_program_address(&[b"collection", collection.as_ref()], &turbin3_program);

    // Create a new mint for the NFT
    let mint = Keypair::new();

    // Discriminator for `submit_rs`
    let data = vec![77, 124, 82, 163, 21, 133, 181, 206];

    let accounts = vec![
        AccountMeta::new(signer_pubkey, true),
        AccountMeta::new(account_pda, false),
        AccountMeta::new(mint.pubkey(), true),
        AccountMeta::new(collection, false),
        AccountMeta::new_readonly(authority_pda, false),
        AccountMeta::new_readonly(mpl_core_program, false),
        AccountMeta::new_readonly(system_program, false),
    ];

    let rpc_client = RpcClient::new(RPC_URL);
    let blockhash = rpc_client.get_latest_blockhash().expect("Failed to fetch blockhash");

    let instruction = Instruction {
        program_id: turbin3_program,
        accounts,
        data,
    };

    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&signer_pubkey),
        &[&signer, &mint],
        blockhash,
    );

    let signature = rpc_client
        .send_and_confirm_transaction(&tx)
        .expect("Transaction failed");

    println!(
        "✅ submit_rs() successful! TX: https://explorer.solana.com/tx/{}?cluster=devnet",
        signature
    );
}


#[test]
fn print_pda_and_mint() {
    use solana_sdk::signature::read_keypair_file;
    use solana_program::pubkey::Pubkey;
    use std::str::FromStr;
    use solana_sdk::signature::Signer;


    // Load the same wallet used in submit_ts()
    let signer = read_keypair_file("Turbin3-wallet.json").expect("Missing wallet");
    let signer_pubkey = signer.pubkey();

    let turbin3_program = Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
    let collection = Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();

    // Derive the account PDA used in both submit_ts and submit_rs
    let (account_pda, _) = Pubkey::find_program_address(&[b"prereqs", signer_pubkey.as_ref()], &turbin3_program);
    
    // Derive authority PDA for collection
    let (authority_pda, _) = Pubkey::find_program_address(&[b"collection", collection.as_ref()], &turbin3_program);

    println!("📍 PDA (accountKey): {}", account_pda);
    println!("🔐 Signer pubkey (user): {}", signer_pubkey);
    println!("🏛️ Authority PDA: {}", authority_pda);
}



}
