import * as anchor from "@coral-xyz/anchor";
import { Program, web3, BN } from "@coral-xyz/anchor";
import { Favorites } from "../target/types/favorites";
import { expect, describe, test } from "@jest/globals";

describe("favorites", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const program = anchor.workspace.Favorites as Program<Favorites>;

  test("Set and update favorites", async () => {
    const user = web3.Keypair.generate();

    const favoriteNumber = new BN(23);
    const favoriteColor = "red";

    // Airdrop SOL to the user
    await provider.connection.requestAirdrop(user.publicKey, web3.LAMPORTS_PER_SOL);

    // Calculate PDA for the favorites account
    const [favoritesPda] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("favorites"), user.publicKey.toBuffer()],
      program.programId
    );

    // 1️⃣ Set initial favorites
    await program.methods
      .setFavorites(favoriteNumber, favoriteColor)
      .accounts({
        favorites: favoritesPda,
        user: user.publicKey,
        //systemProgram: web3.SystemProgram.programId, 
      })
      .signers([user])
})
})



// import * as anchor from "@coral-xyz/anchor";
// import { Program, web3, BN } from "@coral-xyz/anchor";
// import { Favorites } from "../target/types/favorites";
// import { airdropIfRequired, getCustomErrorMessage } from "@solana-developers/helpers";
// import { expect, describe, test } from '@jest/globals';
// import { systemProgramErrors } from "./system-program-errors";

// describe("favorites", () => {
//   anchor.setProvider(anchor.AnchorProvider.env());
//   const provider = anchor.getProvider();
//   const program = anchor.workspace.Favorites as Program<Favorites>;

//   test("Create and update favorites", async () => {
//     const user = web3.Keypair.generate();

//     const favoriteNumber = new BN(23);
//     const favoriteColor = "red";

//     await airdropIfRequired(
//       provider.connection,
//       user.publicKey,
//       0.5 * web3.LAMPORTS_PER_SOL,
//       1 * web3.LAMPORTS_PER_SOL
//     );

//     // Call setFavorites to initialize the PDA account
//     await program.methods
//       .setFavorites(favoriteNumber, favoriteColor)
//       .accounts({
//         user: user.publicKey,
//       })
//       .signers([user])
//       .rpc();

//     const [favoritesPda] = web3.PublicKey.findProgramAddressSync(
//       [Buffer.from("favorites"), user.publicKey.toBuffer()],
//       program.programId
//     );

//     // Confirm initial values
//     let data = await program.account.favorites.fetch(favoritesPda);
//     expect(data.number.toNumber()).toBe(23);
//     expect(data.color).toBe("red");

//     // Now update only the number
//     const updatedNumber = new BN(77);
//     await program.methods
//       .updateFavorites(updatedNumber, null)
//       .accounts({
//         user: user.publicKey,
//         favorites: favoritesPda,
//       })
//       .signers([user])
//       .rpc();

//     data = await program.account.favorites.fetch(favoritesPda);
//     expect(data.number.toNumber()).toBe(77);
//     expect(data.color).toBe("red");

//     // Now update only the color
//     const updatedColor = "blue";
//     await program.methods
//     .updateFavorites({ some: null }, { some: updatedColor })   
//      .accounts({
//         user: user.publicKey,
//         //favorites: favoritesPda,
//       })
//       .signers([user])
//       .rpc();

//     data = await program.account.favorites.fetch(favoritesPda);
//     expect(data.number.toNumber()).toBe(77);
//     expect(data.color).toBe("blue");

//     // Now update both fields
//     const finalNumber = new BN(100);
//     const finalColor = "green";
//     await program.methods
//       .updateFavorites(some: finalNumber, some: finalColor)
//       .accounts({
//         user: user.publicKey,
//         //favorites: favoritesPda,
//       })
//       .signers([user])
//       .rpc();

//     data = await program.account.favorites.fetch(favoritesPda);
//     expect(data.number.toNumber()).toBe(100);
//     expect(data.color).toBe("green");
//   });
// });





// import * as anchor from "@coral-xyz/anchor";
// import { Program, web3 } from "@coral-xyz/anchor";
// import { Favorites } from "../target/types/favorites";
// import { airdropIfRequired, getCustomErrorMessage } from "@solana-developers/helpers";
// import { expect, describe, test } from '@jest/globals';
// import { systemProgramErrors } from "./system-program-errors";

// describe("favorites", () => {
//   // Configure the client to use the local cluster.
//   anchor.setProvider(anchor.AnchorProvider.env());

//   it("Is initialized!", async () => {
//     const user = web3.Keypair.generate();
//     // Here's what we want to write to the blockchain
//     const favoriteNumber = new anchor.BN(23);
//     const favoriteColor = "red";

//     // Add your test here.
//     //const program = anchor.workspace.favorites;
//     //const tx = await program.methods.initialize().rpc();
//     //console.log("YWrites our favorites to the blockchain", tx);

//     const program = anchor.workspace.Favorites as Program<Favorites>;

//     console.log(`User public key: ${user.publicKey}`);

//     await airdropIfRequired(
//       anchor.getProvider().connection,
//       user.publicKey,
//       0.5 * web3.LAMPORTS_PER_SOL,
//       1 * web3.LAMPORTS_PER_SOL
//     );

//     // Make a transaction to write to the blockchain
//     let tx: string | null = null;
//     try {
//       tx = await program.methods
//         // Call the set_favorites instruction handler
//         .setFavorites(favoriteNumber, favoriteColor)
//         .accounts({
//           user: user.publicKey,
//           // Note that both `favorites` and `system_program` are added
//           // automatically.
//         })
//         // Sign the transaction
//         .signers([user])
//         // Send the transaction to the cluster or RPC
//         .rpc();
//     } catch (thrownObject) {
//       // Let's properly log the error, so we can see the program involved
//       // and (for well known programs) the full log message.

//       const rawError = thrownObject as Error;
//       throw new Error(getCustomErrorMessage(systemProgramErrors, rawError.message));
//     }

//     console.log(`Tx signature: ${tx}`);

//      // Calculate the PDA account address that holds the user's favorites
//     const [favoritesPda, _favoritesBump] = web3.PublicKey.findProgramAddressSync(
//       [Buffer.from("favorites"), user.publicKey.toBuffer()],
//       program.programId
//    );

//     // And make sure it matches!
//     const dataFromPda = await program.account.favorites.fetch(favoritesPda);
//     expect(dataFromPda.color).toEqual(favoriteColor);
//     expect(dataFromPda.number.toNumber()).toEqual(favoriteNumber.toNumber());
//   });
// });
