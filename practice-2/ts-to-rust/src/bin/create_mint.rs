use dotenv::dotenv;
use std::{env, error::Error};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{signature::{Keypair, Signer}, commitment_config::CommitmentConfig};
use spl_token::instruction::initialize_mint;
use spl_token::state::Mint;
use solana_sdk::pubkey::Pubkey;
use solana_program::program_pack::Pack;
use solana_sdk::system_instruction::create_account;
use solana_sdk::transaction::Transaction;
use solana_sdk::system_program;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_program::instruction::Instruction;
use spl_token::id as token_program_id;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("token mint...");
    create_token_mint().await?;
    Ok(())
}

async fn create_token_mint() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let secret_key_str = env::var("SECRET_KEY").expect("SECRET_KEY not found");
    let secret_key_bytes: Vec<u8> = serde_json::from_str(&secret_key_str)?;
    let keypair = Keypair::from_bytes(&secret_key_bytes)?;

    let rpc_url = "https://api.devnet.solana.com";
    let connection = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    println!("🔑 Public key is: {}", keypair.pubkey());

    let mint_keypair = Keypair::new();
    let mint_pubkey = mint_keypair.pubkey();

    let mint_rent = connection.get_minimum_balance_for_rent_exemption(Mint::LEN)?;

    let create_account_ix = create_account(
        &keypair.pubkey(),
        &mint_pubkey,
        mint_rent,
        Mint::LEN as u64,
        &token_program_id(),
    );

    let initialize_mint_ix = initialize_mint(
        &token_program_id(),
        &mint_pubkey,
        &keypair.pubkey(),
        None,
        2,
    )?;

    let recent_blockhash = connection.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[create_account_ix, initialize_mint_ix],
        Some(&keypair.pubkey()),
        &[&keypair, &mint_keypair],
        recent_blockhash,
    );

    let signature = connection.send_and_confirm_transaction(&tx)?;
    let explorer_url = format!("https://explorer.solana.com/address/{}?cluster=devnet", mint_pubkey);

    println!("✅ Token Mint created: {}", explorer_url);
    println!("🔗 Transaction Signature: {}", signature);

    Ok(())
}

//1  H3drU5wfMAn5cjFQrUnDu6VDaKExxKEp5Zbu5XXyiiR4
//2  FK9WryedFTmFjCBQZEZZgFsUhnqGY2mVDNkfjcm5gRQy