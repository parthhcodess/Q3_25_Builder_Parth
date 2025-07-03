import wallet from "/home/parth/wallet/my_wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { 
    createMetadataAccountV3, 
    CreateMetadataAccountV3InstructionAccounts, 
    CreateMetadataAccountV3InstructionArgs,
    DataV2Args
} from "@metaplex-foundation/mpl-token-metadata";
import { createSignerFromKeypair, signerIdentity, publicKey } from "@metaplex-foundation/umi";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";

// Define our Mint address
<<<<<<< HEAD
const mint = publicKey("AaS3mQPS6sScGtJ8ABxvg4vcyWkG1pMtt5y2Uu1BcYuv")
=======
const mint = publicKey("3cZSGM6J8Fuu7RFWWoJDjd7dmr6cWoKHfLoNQ6deAM9d")
>>>>>>> 35f44b2 (rug day)

// Create a UMI connection
const umi = createUmi('https://api.devnet.solana.com');
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));

(async () => {
    try {
        // Start here
        let accounts: CreateMetadataAccountV3InstructionAccounts = {
            mint,
            mintAuthority: signer,
<<<<<<< HEAD

        }

        let data: DataV2Args = {
            name: "skullfighter",
            symbol: "SKF",
            uri: "https://arweave.net",
=======
        }

        let data: DataV2Args = {
            name: "Spidey",
            symbol: "SPD",
            uri: "https://gateway.pinata.cloud/ipfs/bafkreiexgixt2tilkfpxb3iopqm7don5zhb3plwbsiuwy2nmivzl55zrxm",
>>>>>>> 35f44b2 (rug day)
            sellerFeeBasisPoints: 1,
            creators: null,
            collection: null,
            uses: null
        }

        let args: CreateMetadataAccountV3InstructionArgs = {
            data: data,
            isMutable: true,
            collectionDetails: null
        }

        let tx = createMetadataAccountV3(
            umi,
            {
                ...accounts,
                ...args
            }
        )

        let result = await tx.sendAndConfirm(umi);
        console.log(bs58.encode(result.signature));
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();
