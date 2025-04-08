use dotenv::dotenv;
use std::{env, error::Error, str::FromStr};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use spl_associated_token_account::instruction::create_associated_token_account;
use spl_associated_token_account::get_associated_token_address;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("token mint...");
    create_token_account().await?;
    Ok(())
}

async fn create_token_account() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let secret_key_str = env::var("SECRET_KEY").expect("SECRET_KEY not found in .env");
    let secret_key_bytes: Vec<u8> = serde_json::from_str(&secret_key_str)?;
    let sender = Keypair::from_bytes(&secret_key_bytes)?;

    let rpc_url = "https://api.devnet.solana.com";
    let connection = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    println!("🔑 Sender Public Key: {}", sender.pubkey());

    let token_mint = Pubkey::from_str("FK9WryedFTmFjCBQZEZZgFsUhnqGY2mVDNkfjcm5gRQy")?;
    let recipient = Pubkey::from_str("C7Bcb8j7CWx8rUhm8JvVJofmrFbU9u7xfYccTEREzXc2")?;

    let associated_token_address = spl_associated_token_account::get_associated_token_address(&recipient, &token_mint);

    let create_ata_ix = create_associated_token_account(
        &sender.pubkey(),  
        &recipient,  
        &token_mint,  
        &spl_token::id(),
    );

    let recent_blockhash = connection.get_latest_blockhash()?;
    
    let tx = Transaction::new_signed_with_payer(
        &[create_ata_ix],
        Some(&sender.pubkey()),
        &[&sender],
        recent_blockhash,
    );

    let sig = connection.send_and_confirm_transaction(&tx)?;

    println!("✅ Token Account Created: {}", associated_token_address);
    println!("🔗 Explorer: https://explorer.solana.com/address/{}?cluster=devnet", associated_token_address);
    println!("🔗 Transaction Signature: {}", sig);

    Ok(())
}

//6wa9Nx4BefNF9Kvw4Z669LjRSEQcTGmcJZBJ92LsxBsu

//6NPHwAmCbjo2AoNLDNFZNWP4sgV6gA9JJAjwyG4WXt7H
