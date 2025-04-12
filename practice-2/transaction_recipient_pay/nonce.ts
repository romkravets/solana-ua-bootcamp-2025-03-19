import "dotenv/config";
import bs58 from "bs58";
import {
  createMint,
  createMultisig,
  getOrCreateAssociatedTokenAccount,
  createMintToInstruction,
} from "@solana/spl-token";
import {
  clusterApiUrl,
  Connection,
  Keypair,
  Transaction,
  SystemProgram,
  PublicKey,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";

(async () => {
  try {
    const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

    // Validate and load secret keys
    const loadKeypair = (keyName) => {
      const keyData = process.env[keyName];
      if (!keyData) {
        throw new Error(`Environment variable ${keyName} is missing`);
      }
      try {
        const parsedKey = JSON.parse(keyData);
        if (!Array.isArray(parsedKey) || parsedKey.length !== 64) {
          throw new Error(`${keyName} is not a valid 64-byte key array`);
        }
        return Uint8Array.from(parsedKey);
      } catch (e) {
        throw new Error(`Failed to parse ${keyName}: ${e.message}`);
      }
    };

    const payer = Keypair.fromSecretKey(loadKeypair("SECRET_KEY"));
    console.log("💸 Payer Public Key:", payer.publicKey.toBase58());

    const signer1 = Keypair.fromSecretKey(loadKeypair("SIGNER1_SECRET_KEY"));
    const signer2 = Keypair.fromSecretKey(loadKeypair("SIGNER2_SECRET_KEY"));
    console.log("👥 Signer 1 Public Key:", signer1.publicKey.toBase58());
    console.log("👥 Signer 2 Public Key:", signer2.publicKey.toBase58());

    // Check payer balance
    const balance = await connection.getBalance(payer.publicKey);
    console.log("Payer SOL Balance:", balance / LAMPORTS_PER_SOL);
    if (balance < 0.1 * LAMPORTS_PER_SOL) {
      throw new Error("Payer has insufficient SOL. Need at least 0.1 SOL.");
    }

    // Create nonce account
    const nonceAccount = Keypair.generate();
    const nonceAccountBalance = await connection.getMinimumBalanceForRentExemption(80);
    const createNonceAccountTransaction = new Transaction().add(
      SystemProgram.createAccount({
        fromPubkey: payer.publicKey,
        newAccountPubkey: nonceAccount.publicKey,
        lamports: nonceAccountBalance,
        space: 80,
        programId: SystemProgram.programId,
      }),
      SystemProgram.nonceInitialize({
        noncePubkey: nonceAccount.publicKey,
        authorizedPubkey: payer.publicKey,
      })
    );

    const nonceCreateSignature = await connection.sendTransaction(
      createNonceAccountTransaction,
      [payer, nonceAccount],
      { skipPreflight: false }
    );
    await connection.confirmTransaction(nonceCreateSignature, "confirmed");
    console.log("✅ Nonce Account Created:", nonceAccount.publicKey.toBase58());

    // Fetch nonce account data
    const nonceAccountData = await connection.getAccountInfo(nonceAccount.publicKey);
    if (!nonceAccountData || !nonceAccountData.data) {
      throw new Error("Failed to fetch nonce account data");
    }
    console.log("Nonce Account Data Length:", nonceAccountData.data.length);

    // Validate nonce account data
    if (!(nonceAccountData.data instanceof Uint8Array)) {
      throw new Error("Nonce account data is not a Uint8Array");
    }

    // Parse nonce data
    let nonceBlockhash;
    try {
      const nonceBytes = nonceAccountData.data.slice(44, 76);
      if (nonceBytes.length !== 32) {
        throw new Error("Invalid nonce field length");
      }
      nonceBlockhash = bs58.encode(nonceBytes);
      console.log("Parsed Nonce Blockhash:", nonceBlockhash);
    } catch (e) {
      console.warn(`Failed to parse nonce: ${e.message}. Falling back to recent blockhash.`);
      const { blockhash } = await connection.getLatestBlockhash("finalized");
      nonceBlockhash = blockhash;
      console.log("Fallback Blockhash:", nonceBlockhash);
    }

    // Create multisig
    const multisig = await createMultisig(
      connection,
      payer,
      [signer1.publicKey, signer2.publicKey],
      2
    );
    console.log("✅ Multisig Account Created:", multisig.toBase58());

    // Create new mint
    const mint = await createMint(
      connection,
      payer,
      multisig,
      null,
      9
    );
    console.log("✅ Token Mint Created:", mint.toBase58());

    // Create receiver token account
    const receiver = Keypair.fromSecretKey(loadKeypair("RECEIVER_SECRET_KEY_2"));
    console.log("👤 Receiver Public Key:", receiver.publicKey.toBase58());
    const receiverTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      payer,
      mint,
      receiver.publicKey
    );
    console.log("🏦 Receiver Token Account:", receiverTokenAccount.address.toBase58());

    // Create mint instruction
    const amountToMint = 100 * 10 ** 9;
    const mintInstruction = createMintToInstruction(
      mint,
      receiverTokenAccount.address,
      multisig,
      amountToMint,
      [signer1, signer2]
    );

    // Create transaction
    const nonceAdvanceInstruction = SystemProgram.nonceAdvance({
      noncePubkey: nonceAccount.publicKey,
      authorizedPubkey: payer.publicKey,
    });

    const transaction = new Transaction().add(nonceAdvanceInstruction, mintInstruction);
    transaction.recentBlockhash = nonceBlockhash;
    transaction.feePayer = payer.publicKey;

    // Sign transaction
    transaction.partialSign(payer, signer1, signer2);
    console.log(
      "Transaction Signatures:",
      transaction.signatures.map((sig) => sig.publicKey.toBase58())
    );

    // Simulate transaction
    const simulation = await connection.simulateTransaction(transaction);
    if (simulation.value.err) {
      console.error("Simulation failed:", simulation.value.logs);
      throw new Error("Transaction simulation failed");
    }
    console.log("Simulation succeeded:", simulation.value.logs);

    // Send transaction with retry logic
    let signature;
    let attempts = 0;
    const maxAttempts = 3;
    while (attempts < maxAttempts) {
      try {
        signature = await connection.sendRawTransaction(transaction.serialize(), {
          skipPreflight: false,
        });
        break;
      } catch (e) {
        attempts++;
        console.warn(`Send attempt ${attempts} failed: ${e.message}`);
        if (attempts === maxAttempts) {
          throw new Error(`Failed to send transaction after ${maxAttempts} attempts: ${e.message}`);
        }
        await new Promise((resolve) => setTimeout(resolve, 1000)); // Wait 1s before retry
      }
    }

    // Confirm transaction
    try {
      await connection.confirmTransaction(signature, "confirmed");
      console.log(
        `✅ Tokens minted successfully: https://explorer.solana.com/tx/${signature}?cluster=devnet`
      );
    } catch (e) {
      console.error(`Confirmation failed: ${e.message}`);
      const txDetails = await connection.getTransaction(signature, {
        commitment: "confirmed",
      });
      console.log("Transaction Details:", txDetails);
      throw e;
    }
  } catch (error) {
    console.error("Transaction failed:", error);
    if (error.logs) {
      console.error("Transaction Logs:", error.logs);
    }
    throw error;
  }
})();