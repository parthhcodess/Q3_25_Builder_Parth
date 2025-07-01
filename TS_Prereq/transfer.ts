import { Transaction, SystemProgram, Connection, Keypair, LAMPORTS_PER_SOL, sendAndConfirmTransaction, PublicKey } from "@solana/web3.js"
import wallet from "./dev-wallet.json"

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet["Private Key"]));
const to = new PublicKey("4UxpHTgUorzAD3pEAzpNi8TGEesZr6xGoiZ95ADUpnYu");

const connection = new Connection("https://api.devnet.solana.com");

(async () => {
  try {
    // Get current balance
    const balance = await connection.getBalance(keypair.publicKey);
    console.log(`Current balance: ${balance / LAMPORTS_PER_SOL} SOL`);

    // Create a dummy transaction to estimate fees
    let transaction = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: keypair.publicKey,
        toPubkey: to,
        lamports: balance, // just for estimation
      })
    );

    transaction.recentBlockhash = (await connection.getLatestBlockhash('confirmed')).blockhash;
    transaction.feePayer = keypair.publicKey;

    // Calculate actual transaction fee
    const fee = (await connection.getFeeForMessage(transaction.compileMessage(), 'confirmed')).value || 0;

    console.log(`Estimated fee: ${fee} lamports`);

    // Remove dummy instruction and replace with actual
    transaction = new Transaction(); // Reset
    transaction.add(
      SystemProgram.transfer({
        fromPubkey: keypair.publicKey,
        toPubkey: to,
        lamports: balance - fee,
      })
    );
    transaction.recentBlockhash = (await connection.getLatestBlockhash('confirmed')).blockhash;
    transaction.feePayer = keypair.publicKey;

    // Sign and send transaction
    const signature = await sendAndConfirmTransaction(connection, transaction, [keypair]);

    console.log(`✅ Success! Check out your TX here:
https://explorer.solana.com/tx/${signature}?cluster=devnet`);
  } catch (e) {
    console.error(`❌ Oops, something went wrong:\n`, e);
  }
})();