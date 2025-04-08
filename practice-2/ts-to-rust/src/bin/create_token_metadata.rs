// use dotenv::dotenv;
// use std::{env, error::Error, str::FromStr};

// use solana_client::rpc_client::RpcClient;
// use solana_sdk::{
//     commitment_config::CommitmentConfig,
//     pubkey::Pubkey,
//     signature::{Keypair, Signer},
//     transaction::Transaction,
//     system_program,
// };

// use mpl_token_metadata::{
//     instruction::create_metadata_accounts_v3,
//     state::DataV2,
// };

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     println!("Creating token metadata...");
//     create_token_metadata().await?;
//     Ok(())
// }

// async fn create_token_metadata() -> Result<(), Box<dyn Error>> {
//     // Load environment variables
//     dotenv().ok();

//     // Load private key from environment
//     let secret_key_str = env::var("SECRET_KEY").expect("SECRET_KEY not found in .env");
//     let secret_key_bytes: Vec<u8> = serde_json::from_str(&secret_key_str)?;
//     let user = Keypair::from_bytes(&secret_key_bytes)?;

//     // Connect to Solana devnet
//     let rpc_url = "https://api.devnet.solana.com";
//     let connection = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

//     println!("🔑 Our public key is: {}", user.pubkey());

//     // Define constants
//     let token_mint_account = Pubkey::from_str("Ajb4desiF2H1W4vv3cQsS8Sku7PKDLvPUDPjViRAJ8ZN")?;
//     let token_metadata_program_id = Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")?;

//     // Derive Metadata PDA
//     let (metadata_pda, _bump) = Pubkey::find_program_address(
//         &[
//             b"metadata",
//             token_metadata_program_id.as_ref(),
//             token_mint_account.as_ref(),
//         ],
//         &token_metadata_program_id,
//     );

//     // Build metadata
//     let metadata_data = DataV2 {
//         name: "ROM-12".to_string(),
//         symbol: "RK-12".to_string(),
//         uri: "https://ipfs.io/ipfs/bafkreieugoauwrhdazkhcsyjsgomv6hvcrucabunwrmc6obirs36cvmzqq".to_string(),
//         seller_fee_basis_points: 0,
//         creators: None,
//         collection: None,
//         uses: None,
//     };

//     // Create metadata instruction
//     let create_metadata_ix = create_metadata_accounts_v3(
//         token_metadata_program_id,
//         metadata_pda,
//         user.pubkey(),       // mint authority
//         user.pubkey(),       // payer
//         user.pubkey(),       // update authority
//         token_mint_account,
//         metadata_data,
//         true,  // is_mutable
//         true,  // update_authority_is_signer
//         None,  // collection_details
//     );

//     // Build transaction
//     let recent_blockhash = connection.get_latest_blockhash()?;
//     let tx = Transaction::new_signed_with_payer(
//         &[create_metadata_ix],
//         Some(&user.pubkey()),
//         &[&user],
//         recent_blockhash,
//     );

//     // Send and confirm
//     let sig = connection.send_and_confirm_transaction(&tx)?;

//     println!("✅ Metadata added!");
//     println!(
//         "🔗 Explorer: https://explorer.solana.com/address/{}?cluster=devnet",
//         token_mint_account
//     );
//     println!("🔗 Transaction Signature: {}", sig);

//     Ok(())
// }
