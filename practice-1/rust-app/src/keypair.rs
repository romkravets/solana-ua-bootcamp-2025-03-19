use solana_sdk::signer::keypair::Keypair;
use std::env;
use dotenv::dotenv;

pub fn load_keypair_from_env() -> Keypair {
    dotenv().ok();

    let secret_key_str = env::var("SECRET_KEY").expect("Environment not found!");
    
    let secret_key_bytes: Vec<u8> = serde_json::from_str(&secret_key_str)
    .expect("Failed to parse SECRET_KEY");

    let keypair = Keypair::from_bytes(&secret_key_bytes)
    .expect("Failed to create Keypair from secret key");

    keypair
}
