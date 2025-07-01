import { Connection, Keypair, PublicKey, SystemProgram, Transaction } from "@solana/web3.js"
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor"
import { IDL, Turbin3Prereq } from "./programs/Turbin3_prereq";
import wallet from "./Turbin3-wallet.json"
import bs58 from "bs58";

const MPL_CORE_PROGRAM_ID = new PublicKey("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");
const mintCollection = new PublicKey("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2");

const keypair = Keypair.fromSecretKey(bs58.decode(wallet["Private Key"]));
const connection = new Connection("https://api.devnet.solana.com");

const provider = new AnchorProvider(connection, new Wallet(keypair), {
commitment: "confirmed"});

const SYSTEM_PROGRAM_ID = new PublicKey("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM");
const program = new Program<Turbin3Prereq>(IDL, provider);


const [accountKey] = PublicKey.findProgramAddressSync(
  [Buffer.from("prereqs"), keypair.publicKey.toBuffer()],
  program.programId
);

const mintTs = Keypair.generate();

// (async () => {
// try {
// const txhash = await program.methods
// .initialize("parthhcodess")
// .accountsPartial({
// user: keypair.publicKey,
// account: accountKey,
// system_program: SYSTEM_PROGRAM_ID,
// })
// .signers([keypair])
// .rpc();
// console.log(`Success! Check out your TX here:
// https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
// } catch (e) {
// console.error(`Oops, something went wrong: ${e}`);
// }
// })();
// Execute the submitTs transaction
(async () => {
try {
const txhash = await program.methods
.submitTs()
.accountsPartial({
user: keypair.publicKey,
account: accountKey,
mint: mintTs.publicKey,
collection: mintCollection,
mpl_core_program: MPL_CORE_PROGRAM_ID,
system_program: SYSTEM_PROGRAM_ID,
})
.signers([keypair, mintTs])
.rpc();
console.log(`Success! Check out your TX here:
https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
} catch (e) {
console.error(`Oops, something went wrong: ${e}`);
}
})();