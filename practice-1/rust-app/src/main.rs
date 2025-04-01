//mod keygen;
mod keypair;
mod balance;
mod solana_transfer;
use solana_sdk::signature::Signer;

fn main() {
    //let (public_key, secret_key) = keygen::generate_keypair();
    //println!("✅ Public Key: {}", public_key);
    //println!("✅ Public Key: {}", public_key)
    let keypair = keypair::load_keypair_from_env();
    balance::check_balance_and_airdrop();
    solana_transfer::send_sol();
    println!("✅ Public Key: {}", keypair.pubkey());
    
}
