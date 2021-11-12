
import {
    clusterApiUrl,
    Connection,
    Keypair,
    LAMPORTS_PER_SOL, PublicKey, sendAndConfirmTransaction,
    Transaction,
    TransactionInstruction
} from "@solana/web3.js";
import * as Buffer from "buffer";

const connection = new Connection(clusterApiUrl("devnet"));
const key: Uint8Array = Uint8Array.from([34,21,156,214,135,190,149,175,180,176,205,34,1,192,60,7,220,16,70,70,252,109,57,44,188,116,116,157,117,20,161,188,243,192,183,67,213,220,74,64,143,193,202,197,141,53,234,179,202,115,118,43,78,1,71,158,70,87,110,46,207,3,82,223]);
const programId = new PublicKey("D3FbEaSH2mUPbtkvbr2W6S3sUCFMCqJw1eawA3PPfX3T");

async function main() {
    console.log("Hello world");

    const signer: Keypair = Keypair.fromSecretKey(key);

    console.log("Signer: ", signer.publicKey.toBase58());
    console.log("ProgramId: ", programId.toBase58());

    await connection.getBalance(signer.publicKey).then((balance) => {
        console.log("Balance: ", balance / LAMPORTS_PER_SOL);
    });


    const data: Buffer = Buffer.Buffer.from("blah");

    const tx = new Transaction().add(
      new TransactionInstruction({
          keys: [],
          programId,
          data,
      })
    );

    await sendAndConfirmTransaction(connection,tx,[signer]).then((sig) => {
        console.log("TxId: ", sig);
    });

    console.log("The end");
}

main().then(() => process.exit(0)).catch((error) => {
    console.error(error);
    process.exit(1);
})
