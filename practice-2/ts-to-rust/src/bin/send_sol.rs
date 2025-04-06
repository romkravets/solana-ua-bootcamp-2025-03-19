// src/bin/send_sol.rs
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    system_instruction,
    pubkey::Pubkey,
    instruction::{Instruction, AccountMeta},
    sysvar,
};
use std::str::FromStr;
use dotenv::dotenv;
use std::env;
use std::error::Error;
use solana_client::rpc_client::RpcClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Running send_sol...");
    send_sol().await?;
    Ok(())
}

async fn send_sol() -> Result<(), Box<dyn Error>> {
    dotenv().ok();


    let secret_key_str = env::var("SECRET_KEY").expect("SECRET_KEY not found");
    let secret_key_bytes: Vec<u8> = serde_json::from_str(&secret_key_str).expect("Invalid secret");
    let keypair = Keypair::from_bytes(&secret_key_bytes).expect("Failed to load keypair");

    let connection = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    let rec_pubkey = Pubkey::from_str("Hh2Ho83HMZuch7Tf5rxHqpqHytpzxQX18Yb8MnfVtD5x")?;
    let memo_program = Pubkey::from_str("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr")?;
    let memo_text = "Hello from Solana Roman Kravets!";

    let memo_instruction = Instruction {
        program_id: memo_program,
        accounts: vec![AccountMeta::new_readonly(sysvar::rent::id(), false)],
        data: memo_text.as_bytes().to_vec(),
    };

    let transfer_instruction = system_instruction::transfer(
        &keypair.pubkey(),
        &rec_pubkey,
        5_000_000,
    );

    let mut transaction = Transaction::new_with_payer(
        &[transfer_instruction],
        Some(&keypair.pubkey()),
    );

    let recent_blockhash = connection.get_latest_blockhash()?;
    transaction.try_sign(&[&keypair], recent_blockhash)?;

    let signature = connection.send_and_confirm_transaction_with_spinner(&transaction)?;

    println!("📝 Memo: {}", memo_text);
    println!("✅ Signature: {}", signature);

    Ok(())
}
