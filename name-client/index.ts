import { Connection, Keypair, LAMPORTS_PER_SOL, SystemProgram, Transaction } from "@solana/web3.js";

const connection = new Connection("http://127.0.0.1:8899");

async function main() {
   const kp = new Keypair();
   const dataAccount = new Keypair();
   const signature = await connection.requestAirdrop(kp.publicKey, 3000_000_000);

   await connection.confirmTransaction(signature);
   const balance = await connection.getBalance(kp.publicKey);

   const instruction = SystemProgram.createAccount({
      fromPubkey: kp.publicKey,
      newAccountPubkey: dataAccount.publicKey,
      lamports: 1000_000_000,
      space: 8,
      programId: SystemProgram.programId,
   });

   // how will you send the instruction by using transaction.
   const tx = new Transaction().add(instruction);
   tx.feePayer = kp.publicKey;
   tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
   // tx.sign(kp);
   await connection.sendTransaction(tx,[kp, dataAccount]);

   console.log(dataAccount.publicKey.toBase58());
}
// we wrote a js script to interact with SystemProgram to create a new Account which has 8 bytes of data in it

main();