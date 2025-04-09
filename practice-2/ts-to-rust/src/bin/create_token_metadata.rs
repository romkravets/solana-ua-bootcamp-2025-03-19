use std::env;
use std::error::Error;
use std::str::FromStr;

use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

use mpl_token_metadata::instructions::CreateMetadataAccountV3;
use mpl_token_metadata::types::DataV2;
use mpl_token_metadata::ID as TOKEN_METADATA_PROGRAM_ID;
use mpl_token_metadata::instructions::CreateMetadataAccountV3InstructionArgs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("token mint...");
    create_token_account().await?;
    Ok(())
}

async fn create_token_account() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let secret_key_str = env::var("SECRET_KEY")?;
    let secret_key_bytes: Vec<u8> = serde_json::from_str(&secret_key_str)?;
    let keypair = Keypair::from_bytes(&secret_key_bytes)?;

    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    let mint_pubkey = Pubkey::from_str("FK9WryedFTmFjCBQZEZZgFsUhnqGY2mVDNkfjcm5gRQy")?;

    let metadata_seeds = &[
        b"metadata",
        TOKEN_METADATA_PROGRAM_ID.as_ref(),
        mint_pubkey.as_ref(),
    ];
    let (metadata_pda, _bump) = Pubkey::find_program_address(metadata_seeds, &TOKEN_METADATA_PROGRAM_ID);

    let metadata = DataV2 {
        name: "ROM-12".to_string(),
        symbol: "RK-12".to_string(),
        uri: "https://ipfs.io/ipfs/bafkreieugoauwrhdazkhcsyjsgomv6hvcrucabunwrmc6obirs36cvmzqq".to_string(),
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    let ix = CreateMetadataAccountV3 {
        metadata: metadata_pda,
        mint: mint_pubkey,
        mint_authority: keypair.pubkey(),
        payer: keypair.pubkey(),
        update_authority: (keypair.pubkey(), true),
        system_program: solana_sdk::system_program::id(),
        rent: None,
    }
    .instruction(CreateMetadataAccountV3InstructionArgs {
        data: metadata,
        is_mutable: true,
        collection_details: None,
    });

    let blockhash = client.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&keypair.pubkey()),
        &[&keypair],
        blockhash,
    );

    let sig = client.send_and_confirm_transaction(&tx)?;
    println!("Metadata Tx:", sig);

    Ok(())
}
