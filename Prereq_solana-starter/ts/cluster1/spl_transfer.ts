import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "/home/parth/wallet/my_wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
<<<<<<< HEAD
const mint = new PublicKey("AaS3mQPS6sScGtJ8ABxvg4vcyWkG1pMtt5y2Uu1BcYuv");

// Recipient address
const to = new PublicKey("FVdLp9jrbstgPZDqqWuNbSEKURxigGkigFBRmnCAAksh");
=======
const mint = new PublicKey("3cZSGM6J8Fuu7RFWWoJDjd7dmr6cWoKHfLoNQ6deAM9d");

// Recipient address
const to = new PublicKey("DwUkSRrMWtcxsqVEJk7coMwpRVXDdxS2mxBPjMMgN1pY");
>>>>>>> 35f44b2 (rug day)

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const fromWallet = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey
        )

        // Get the token account of the toWallet address, and if it does not exist, create it
        const toWallet = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            to
        )

        // Transfer the new token to the "toTokenAccount" we just created
        const toTokenAccount = await transfer(
            connection,
            keypair,
            fromWallet.address,
            toWallet.address,
            keypair,
<<<<<<< HEAD
            100000
=======
            10000000
>>>>>>> 35f44b2 (rug day)
        )
        console.log("Transaction Signature:", toTokenAccount);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();