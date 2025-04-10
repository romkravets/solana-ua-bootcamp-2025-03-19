// import "dotenv/config";
// import {
//   Connection,
//   Keypair,
//   PublicKey,
//   SystemProgram,
//   VersionedTransaction,
//   TransactionMessage,
// } from "@solana/web3.js";
// import fs from "fs";

// async function main() {
//   try {
//     // Connect to Solana Devnet
//     const connection = new Connection("https://api.devnet.solana.com", "confirmed");

//     // Load receiver's secret key
//     const receiver = Keypair.fromSecretKey(
//       Uint8Array.from(JSON.parse(process.env["RECEIVER_SECRET_KEY"]!))
//     );
//     console.log("📥 Receiver Public Key:", receiver.publicKey.toBase58());

//     // Define recipient public key
//     const recipientPublicKey = new PublicKey("4GjKR46rtnFgy3TVQAWh2uZX9SRzcTTYxGmXYndKWrLz"); // Replace this

//     // Transfer amount in lamports
//     const transferAmount = 1000000;

//     // Fetch the recent blockhash
//     const latestBlockhash = await connection.getLatestBlockhash();

//     // Create transaction instructions
//     const instructions = [
//       SystemProgram.transfer({
//         fromPubkey: receiver.publicKey,
//         toPubkey: recipientPublicKey,
//         lamports: transferAmount,
//       }),
//     ];

//     // Compile the transaction message for versioned transaction
//     const message = new TransactionMessage({
//       payerKey: receiver.publicKey,
//       instructions,
//       recentBlockhash: latestBlockhash.blockhash,
//     }).compileToV0Message();

//     // Create the Versioned Transaction
//     const tx = new VersionedTransaction(message);

//     // Sign the transaction
//     tx.sign([receiver]);

//     // Serialize and save the transaction
//     const serializedTransaction = tx.serialize().toString("base64");
//     fs.writeFileSync("unsigned_transfer.txn", serializedTransaction);

//     console.log("📤 Transaction serialized and saved to unsigned_transfer.txn");

//     // Send the transaction
//     const sig = await connection.sendRawTransaction(tx.serialize());
//     await connection.confirmTransaction(sig, "confirmed");

//     console.log(`✅ Transaction sent: https://explorer.solana.com/tx/${sig}?cluster=devnet`);
//   } catch (error) {
//     console.error("🚨 Error during transaction:", error);
//   }
// }

// main();

import "dotenv/config";
import {
  Connection,
  Keypair,
  PublicKey,
  Transaction,
  clusterApiUrl,
} from "@solana/web3.js";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  createTransferInstruction,
} from "@solana/spl-token";

async function main() {
  // Connect to Solana Devnet
  const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

  // Load keypairs for sender and receiver from .env
  const sender = Keypair.fromSecretKey(
    Uint8Array.from(JSON.parse(process.env["SECRET_KEY"]!))
  );
  const receiver = Keypair.fromSecretKey(
    Uint8Array.from(JSON.parse(process.env["RECEIVER_SECRET_KEY"]!))
  );

  console.log("🔐 Sender Public Key:", sender.publicKey.toBase58());
  console.log("📥 Receiver Public Key:", receiver.publicKey.toBase58());

  const mint = new PublicKey("FK9WryedFTmFjCBQZEZZgFsUhnqGY2mVDNkfjcm5gRQy");

  console.log("✅ Token Mint Address:", mint);

  // Step 2: Create Associated Token Accounts (ATA)
  const senderTokenAccount =  new PublicKey("6NPHwAmCbjo2AoNLDNFZNWP4sgV6gA9JJAjwyG4WXt7H");

  console.log("🏦 Sender Token Account:", senderTokenAccount);
  
  const receiverTokenAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    sender, // Fee payer (sender pays fee to create receiver's ATA)
    mint, // Token mint
    receiver.publicKey // Owner of the account
  );
  console.log("🏦 Receiver Token Account:", receiverTokenAccount);

  // Step 3: Mint tokens to the sender's token account
  const amountToMint = 100 * 10 ** 9; // 100 tokens with 9 decimals
  await mintTo(
    connection,
    sender, // Fee payer
    mint, // Token mint
    senderTokenAccount.address, // Destination account
    sender.publicKey, // Mint authority
    amountToMint // Amount to mint
  );
  console.log(`✅ Minted ${amountToMint / 10 ** 9} tokens to sender's account.`);

  // Step 4: Transfer tokens from sender to receiver
  const amountToTransfer = 10 * 10 ** 9; // 10 tokens
  const transferIx = createTransferInstruction(
    senderTokenAccount.address, // Source token account
    receiverTokenAccount.address, // Destination token account
    sender.publicKey, // Owner of the source account
    amountToTransfer // Amount to transfer
  );

  const { blockhash } = await connection.getLatestBlockhash();
  const tx = new Transaction({
    feePayer: sender.publicKey,
    recentBlockhash: blockhash,
  }).add(transferIx);

  tx.sign(sender); // Sender signs the transaction

  const sig = await connection.sendRawTransaction(tx.serialize());
  await connection.confirmTransaction(sig, "confirmed");
  console.log(`✅ Tokens transferred: https://explorer.solana.com/tx/${sig}?cluster=devnet`);
}

main().catch((err) => {
  console.error("🚨 Error during transaction:", err);
});

