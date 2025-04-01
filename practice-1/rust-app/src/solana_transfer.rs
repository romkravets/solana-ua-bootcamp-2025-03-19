use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::Signer,
    system_instruction,
    transaction::Transaction,
    pubkey::Pubkey,
    signer::keypair::Keypair,
};
use dotenv::dotenv;
use std::env;
use std::str::FromStr;

pub fn send_sol() {
    dotenv().ok();

    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    let secret_key_str = env::var("SECRET_KEY").expect("SECRET_KEY not found");
    let secret_key_bytes: Vec<u8> = serde_json::from_str(&secret_key_str).expect("Invalid secret");
    let keypair = Keypair::from_bytes(&secret_key_bytes).expect("Failed to load keypair");

    let recipient_pubkey = Pubkey::from_str("HTDpDKuxL31cKmhpN8Y2hQF8K1gAcsyKATos75YCYzi").expect("Invalid recipient address");

    let lamports = 5_000_000;

    let transfer_ix = system_instruction::transfer(&keypair.pubkey(), &recipient_pubkey, lamports);

    let recent_blockhash = client.get_latest_blockhash().expect("Failed to get recent blockhash");

    let transaction = Transaction::new_signed_with_payer(
        &[transfer_ix],
        Some(&keypair.pubkey()),
        &[&keypair],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction).expect("Transaction failed");
    println!("✅ Transaction successful! Signature: {}", signature);
}
