import "dotenv/config";
import {
    getAccount,
    createMint,
    createMultisig,
    mintTo,
    getOrCreateAssociatedTokenAccount,
  } from "@solana/spl-token";
  import {
    clusterApiUrl,
    Connection,
    Keypair,
    Transaction,
    SystemProgram,
  } from "@solana/web3.js";
  
  (async () => {
    const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
  
    const payer = Keypair.fromSecretKey(
        Uint8Array.from(JSON.parse(process.env["SECRET_KEY"]!))
      );
    const signer1 = Keypair.generate();
    const signer2 = Keypair.generate();
    console.log("💸 Payer Public Key:", payer);
    console.log("👥 Signer 1 Public Key:", signer1.publicKey.toBase58());
    console.log("👥 Signer 2 Public Key:", signer2.publicKey.toBase58());
  
    const airdropSignature = await connection.requestAirdrop(
      payer.publicKey,
      2 * 10 ** 9
    );
    //await connection.confirmTransaction(airdropSignature);
  
    const nonceAccount = Keypair.generate();
    const nonceAccountBalance = await connection.getMinimumBalanceForRentExemption(
      76
    );
    const createNonceTransaction = new Transaction().add(
      SystemProgram.createAccount({
        fromPubkey: payer.publicKey,
        newAccountPubkey: nonceAccount.publicKey,
        lamports: nonceAccountBalance,
        space: 76,
        programId: SystemProgram.programId,
      })
    );
    await connection.sendTransaction(createNonceTransaction, [payer, nonceAccount]);
  
    console.log("✅ Nonce Account Created:", nonceAccount.publicKey.toBase58());

    const nonceAdvanceInstruction = SystemProgram.nonceAdvance({
      noncePubkey: nonceAccount.publicKey,
      authorizedPubkey: payer.publicKey,
    });
  
    const multisig = await createMultisig(
      connection,
      payer,
      [signer1.publicKey, signer2.publicKey],
      2
    );
    console.log("✅ Multisig Account Created:", multisig.toBase58());
  
    const mint = await createMint(
      connection,
      payer,
      multisig,
      null,
      9
    );
    console.log("✅ Token Mint Address:", mint.toBase58());
  
    const receiver = Keypair.generate();
    const receiverTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      payer,
      mint,
      receiver.publicKey
    );
    console.log("🏦 Receiver Token Account:", receiverTokenAccount.address.toBase58());
  
    const amountToMint = 100 * 10 ** 9;
    const mintInstruction = await mintTo(
      connection,
      payer,
      mint,
      receiverTokenAccount.address,
      multisig,
      amountToMint
    );
  
    const transaction = new Transaction().add(
      nonceAdvanceInstruction,
      mintInstruction
    );
    transaction.recentBlockhash = nonceAccount.publicKey;
    transaction.feePayer = payer.publicKey;
  
    transaction.partialSign(payer, signer1, signer2);
  
    const signature = await connection.sendRawTransaction(transaction.serialize());
    await connection.confirmTransaction(signature, "confirmed");
    console.log(`✅ Tokens minted successfully: https://explorer.solana.com/tx/${signature}?cluster=devnet`);
  
    const receiverAccountInfo = await getAccount(connection, receiverTokenAccount.address);
    console.log(`Receiver's token balance: ${receiverAccountInfo.amount}`);
  })();
  