<<<<<<< HEAD
import wallet from "../wba-wallet.json"
=======
import wallet from "/home/parth/wallet/my_wallet.json"
>>>>>>> 35f44b2 (rug day)
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"

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
umi.use(irysUploader({ address: "https://devnet.irys.xyz/" }));
>>>>>>> 35f44b2 (rug day)
umi.use(signerIdentity(signer));

(async () => {
    try {
        //1. Load image
        //2. Convert image to generic file.
        //3. Upload image

<<<<<<< HEAD
        // const image = ???

        // const [myUri] = ??? 
        // console.log("Your image URI: ", myUri);
=======
        const image = await readFile("/home/parth/Downloads/andre.png")

        const genericFile = createGenericFile(image ,'andre.png', {
            contentType: 'image/png'
        })

        const [myUri] = await umi.uploader.upload([genericFile])
        console.log("Your image URI: ", myUri);
>>>>>>> 35f44b2 (rug day)
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
