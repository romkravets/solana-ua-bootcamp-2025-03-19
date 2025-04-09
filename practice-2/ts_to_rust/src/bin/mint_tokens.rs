use dotenv::dotenv;
use std::{env, error::Error, str::FromStr};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use spl_token::instruction::mint_to;
use spl_token::state::Mint;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    println!("🚀 Minting tokens...");
    mint_tokens()?;
    Ok(())
}

fn mint_tokens() -> Result<(), Box<dyn Error>> {

    let secret_key_str = env::var("SECRET_KEY").expect("SECRET_KEY not found in .env");
    let secret_key_bytes: Vec<u8> = serde_json::from_str(&secret_key_str)?;
    let sender = Keypair::from_bytes(&secret_key_bytes)?;

    let rpc_url = "https://api.devnet.solana.com";
    let connection = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    println!("🔑 Sender Public Key: {}", sender.pubkey());

    let token_mint = Pubkey::from_str("FK9WryedFTmFjCBQZEZZgFsUhnqGY2mVDNkfjcm5gRQy")?;
    let recipient_associated_token_account = Pubkey::from_str("6NPHwAmCbjo2AoNLDNFZNWP4sgV6gA9JJAjwyG4WXt7H")?;

    let minor_units_per_major_units = 10u64.pow(2);
    let mint_amount = 10 * minor_units_per_major_units;

    let mint_to_ix = mint_to(
        &spl_token::id(),
        &token_mint,
        &recipient_associated_token_account,
        &sender.pubkey(),
        &[],
        mint_amount,
    )?;

    let recent_blockhash = connection.get_latest_blockhash()?;

    let tx = Transaction::new_signed_with_payer(
        &[mint_to_ix],
        Some(&sender.pubkey()),
        &[&sender],
        recent_blockhash,
    );

    let sig = connection.send_and_confirm_transaction(&tx)?;

    println!("Success! Tokens minted.");
    println!("Explorer: {}", sig);

    Ok(())
}
