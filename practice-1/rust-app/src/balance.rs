use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
};
use dotenv::dotenv;
use std::env;
use std::str::FromStr;

pub fn check_balance_and_airdrop() {
    dotenv().ok();

    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    let public_key_str = env::var("PUBLIC_KEY").expect("PUBLIC_KEY not found");
    let public_key = Pubkey::from_str(&public_key_str).expect("Invalid public key format");

    let balance_in_lamports = client.get_balance(&public_key).expect("Failed to get balance");
    let balance_in_sol = balance_in_lamports as f64 / LAMPORTS_PER_SOL as f64;

    println!(
        "The balance for the wallet at address {} is: {} SOL",
        public_key, balance_in_sol
    );

    let min_balance = (0.5 * LAMPORTS_PER_SOL as f64) as u64;
    if balance_in_lamports < min_balance {
        let signature = client.request_airdrop(&public_key, LAMPORTS_PER_SOL).expect("Airdrop failed");
        println!("✅ Airdrop successful: {}", signature);
    }
}
