use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;
use bs58;

pub fn generate_keypair() -> (String, String) {
    let keypair = Keypair::new();
    let public_key = keypair.pubkey().to_string();
    let secret_key = bs58::encode(keypair.to_bytes()).into_string();

    (public_key, secret_key)
}
