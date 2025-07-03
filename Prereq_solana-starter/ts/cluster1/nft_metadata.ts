<<<<<<< HEAD
import wallet from "../wba-wallet.json"
=======
import wallet from "/home/parth/wallet/my_wallet.json"
>>>>>>> 35f44b2 (rug day)
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
<<<<<<< HEAD
const umi = createUmi('https://api.devnet.solana.com');
=======
const umi = createUmi('https://solana-devnet.api.syndica.io/api-key/3nwfWkMbDcGpUrAFQYofpGgaKUK8LJAb3pZrvfmxm5UEZwsu7BYrkHytmK5BYttGtPh9cVw9EH665TstXvc6VoAMLdo8tpKsNVn');
>>>>>>> 35f44b2 (rug day)

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

<<<<<<< HEAD
umi.use(irysUploader());
=======
umi.use(irysUploader({address: "https://devnet.irys.xyz/",}));
>>>>>>> 35f44b2 (rug day)
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

<<<<<<< HEAD
        // const image = ???
        // const metadata = {
        //     name: "?",
        //     symbol: "?",
        //     description: "?",
        //     image: "?",
        //     attributes: [
        //         {trait_type: '?', value: '?'}
        //     ],
        //     properties: {
        //         files: [
        //             {
        //                 type: "image/png",
        //                 uri: "?"
        //             },
        //         ]
        //     },
        //     creators: []
        // };
        // const myUri = ???
        // console.log("Your metadata URI: ", myUri);
=======
        const image = "https://arweave.net/DCYrfBEFtatrMZzgCmA57dC2aPCrZmoNWuhBbrgs7CNS"
        const metadata = {
            name: "THE ANDRE",
            symbol: "ADR",
            description: "The Goofy Andre NFT we all should have",
            image: image,
            attributes: [
                {trait_type: 'Legendary', value: '100$'}
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: "image"
                    },
                ]
            },
            creators: ["skullfighter"]
        };
        const myUri = await umi.uploader.uploadJson(metadata)
        console.log("Your metadata URI: ", myUri);
>>>>>>> 35f44b2 (rug day)
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
