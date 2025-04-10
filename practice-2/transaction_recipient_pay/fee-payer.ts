// import "dotenv/config";
// import {
//   Connection,
//   Keypair,
//   PublicKey,
//   Transaction,
//   clusterApiUrl,
//   sendAndConfirmTransaction,
// } from "@solana/web3.js";
// import {
//   getOrCreateAssociatedTokenAccount,
//   createTransferInstruction,
// } from "@solana/spl-token";
// import fs from "fs";

// async function main() {
//   // Load sender's private key from .env
//   let privateKey = process.env["SECRET_KEY"];
//   if (!privateKey) {
//     console.log("❌ Add SECRET_KEY to your .env file!");
//     process.exit(1);
//   }

//   const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

//   const senderSecretKey = Uint8Array.from(JSON.parse(privateKey));
//   const sender = Keypair.fromSecretKey(senderSecretKey);
//   console.log("🔐 Sender:", sender.publicKey.toBase58());

//   const receiverSecretKey = Uint8Array.from(JSON.parse(process.env["RECEIVER_SECRET_KEY"]));
//   const receiver = Keypair.fromSecretKey(receiverSecretKey);
//   console.log("📥 Receiver:", receiver.publicKey.toBase58());

//   // Mint address (token to transfer)
//   const mint = new PublicKey("FK9WryedFTmFjCBQZEZZgFsUhnqGY2mVDNkfjcm5gRQy");

//   // Get or create associated token accounts
//   const senderTokenAccount = await getOrCreateAssociatedTokenAccount(
//     connection,
//     sender,
//     mint,
//     sender.publicKey
//   );

//   const receiverTokenAccount = await getOrCreateAssociatedTokenAccount(
//     connection,
//     sender, // sender pays the fee if creating the ATA
//     mint,
//     receiver.publicKey
//   );

//   console.log("🏦 Sender Token Account:", senderTokenAccount.address.toBase58());
//   console.log("🏦 Receiver Token Account:", receiverTokenAccount.address.toBase58());

//   // Create the transfer instruction (sending 5 tokens)
//   const transferIx = createTransferInstruction(
//     senderTokenAccount.address,
//     receiverTokenAccount.address,
//     sender.publicKey,
//     1 // remember: this is in the smallest unit (depends on mint decimals)
//   );

//   const { blockhash } = await connection.getLatestBlockhash();
//   const tx = new Transaction({
//     feePayer: sender.publicKey,
//     recentBlockhash: blockhash,
//   }).add(transferIx);

//   tx.sign(sender);

//   const sig = await connection.sendRawTransaction(tx.serialize());
//   await connection.confirmTransaction(sig, "confirmed");

//   console.log("✅ Transaction sent and confirmed:");
//   console.log(`🔗 https://explorer.solana.com/tx/${sig}?cluster=devnet`);
// }

// main().catch((err) => {
//   console.error("🚨 Error during transaction:", err);
// });

// sender_prepare.ts
import "dotenv/config";
import {
  Connection,
  Keypair,
  PublicKey,
  Transaction,
  clusterApiUrl,
} from "@solana/web3.js";
import {
  getOrCreateAssociatedTokenAccount,
  createTransferInstruction,
} from "@solana/spl-token";
import fs from "fs";

async function main() {
  const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

  const sender = Keypair.fromSecretKey(Uint8Array.from(JSON.parse(process.env["SECRET_KEY"]!)));
  const receiver = Keypair.fromSecretKey(Uint8Array.from(JSON.parse(process.env["RECEIVER_SECRET_KEY"]!)));

  const mint = new PublicKey("FK9WryedFTmFjCBQZEZZgFsUhnqGY2mVDNkfjcm5gRQy");

  const senderTokenAccount = await getOrCreateAssociatedTokenAccount(connection, sender, mint, sender.publicKey);
  const receiverTokenAccount = await getOrCreateAssociatedTokenAccount(connection, sender, mint, receiver.publicKey);

  const ix = createTransferInstruction(
    senderTokenAccount.address,
    receiverTokenAccount.address,
    sender.publicKey,
    1
  );

  const receiverPublicKey = new PublicKey(receiver.publicKey); // або просто receiver.publicKey

  const { blockhash } = await connection.getLatestBlockhash();
  const tx = new Transaction({
    feePayer: receiverPublicKey, // ⛽️ Отримувач платить за транзакцію
    recentBlockhash: blockhash,
  }).add(ix);

  tx.partialSign(sender); // 🔏 Лише відправник підписує як власник токенів

  const serializedMessage = tx.serializeMessage(); // тільки message, без підписів
  fs.writeFileSync("unsigned_transfer.txn", serializedMessage.toString("base64"));

  console.log("📝 Transaction prepared and partially signed by sender.");
}

main();

