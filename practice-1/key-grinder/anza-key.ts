import bs58 from "bs58";
import { Keypair } from "@solana/web3.js";

const TARGET_PREFIX = "anza";
let count = 0;

while (true) {
    const keypair = Keypair.generate();
    const publicKey = keypair.publicKey.toBase58();

    count++;

    if (publicKey.startsWith(TARGET_PREFIX)) {
        console.log("key", publicKey);
        console.log("PK (Base58):", bs58.encode(keypair.secretKey));
        console.log(count);
        break;
    }
    if (count % 1000 === 0) {
        console.log(`Check ${count} keys...`);
    }
}
