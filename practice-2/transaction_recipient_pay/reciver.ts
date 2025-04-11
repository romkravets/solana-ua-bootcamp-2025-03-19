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
  const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

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

  const senderTokenAccount =  new PublicKey("6NPHwAmCbjo2AoNLDNFZNWP4sgV6gA9JJAjwyG4WXt7H");

  console.log("🏦 Sender Token Account:", senderTokenAccount);
  
  const receiverTokenAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    sender,
    mint,
    receiver.publicKey
  );
  console.log("🏦 Receiver Token Account:", receiverTokenAccount);

  const amountToMint = 100 * 10 ** 9;
  await mintTo(
    connection,
    sender,
    mint,
    senderTokenAccount,
    sender.publicKey,
    amountToMint
  );
  console.log(`✅ Minted ${amountToMint / 10 ** 9} tokens to sender's account.`);

  const amountToTransfer = 10 * 10 ** 9;
  const transferIx = createTransferInstruction(
    senderTokenAccount,
    receiverTokenAccount.address,
    sender.publicKey,
    amountToTransfer
  );

  const { blockhash } = await connection.getLatestBlockhash();
  const tx = new Transaction({
    feePayer: receiver.publicKey,
    recentBlockhash: blockhash,
  }).add(transferIx);

  console.log("📝 Signing transaction with receiver and sender...");
  tx.sign(receiver, sender);

  console.log(
    "🔍 Transaction signers:",
    tx.signatures.map((s) => s.publicKey.toBase58())
  );

  const sig = await connection.sendRawTransaction(tx.serialize());
  await connection.confirmTransaction(sig, "confirmed");
  console.log(`✅ Tokens transferred: https://explorer.solana.com/tx/${sig}?cluster=devnet`);
}

main().catch((err) => {
  console.error("🚨 Error during transaction:", err);
});