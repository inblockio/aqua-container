import axios from "axios";
import { ethers } from "ethers";

interface IWitnessFile {
    pageVerificationHash: String
    filename: string
}

const WitnessFile = ({ pageVerificationHash, filename }: IWitnessFile) => {

    const witnessFileHandler = async () => {
        if (window.ethereum) {
            try {
                // Connect wallet
                const accounts = await window?.ethereum?.request({ method: 'eth_requestAccounts' });
                const walletAddress = accounts[0];

                if (!walletAddress) {
                    alert("Please connect your wallet to continue");
                    return;
                }

                // Message to sign
                const message = `I sign the following page verification hash: [${pageVerificationHash}]`;

                // Create an ethers provider
                const provider = new ethers.BrowserProvider(window.ethereum);
                const signer = await provider.getSigner();

                // Hash the message (optional but recommended)
                const messageHash = ethers.hashMessage(message);

                // Sign the message using ethers.js
                const signature = await signer.signMessage(message);

                /* This obtains public signing key but it might not be the public key of signer */
                // let pubKey = await window.ethereum.request({
                //     method: "eth_getEncryptionPublicKey",
                //     params: [
                //         walletAddress
                //     ],
                // });
                // console.log("Public key is: 0x", Buffer.from(pubKey, 'base64').toString('hex'))

                if (signature) {
                    try {
                        // Recover the public key from the signature; This returns an address same as wallet address
                        const publicKey = ethers.recoverAddress(messageHash, signature)

                        const formData = new URLSearchParams();
                        formData.append('filename', filename);
                        formData.append('signature', signature);
                        /* Recovered public key if needed */
                        // formData.append('publickey', walletAddress);
                        /* Hardcoded public key value for now; Remove this once a fix for obtaining public keys is found */
                        formData.append('publickey', "0x04c56c1231c8a69a375c3f81e549413eb0f415cfd56d40c9a5622456a3f77be0625e1fe8a50cb6274e5d0959625bf33f3c8d1606b5782064bad2e4b46c5e2a7428");
                        formData.append('wallet_address', ethers.getAddress(walletAddress));

                        const response = await axios.post("http://localhost:3600/explorer_sign_revision", formData, {
                            headers: {
                                'Content-Type': 'application/x-www-form-urlencoded'
                            }
                        });

                        if (response.status === 200) {
                            alert("Revision signed successfully")
                        }
                        console.log(response)

                    } catch (error) {
                        console.error('Error during signature submission:', error);
                    }
                }

            } catch (error) {
                console.error('Error during wallet connection or signing:', error);
            }
        } else {
            alert('MetaMask is not installed');
        }
    };

    return (
        <div  class="rounded bg-primary/25 px-2 mx-4 " style="display: inline-flex; align-items: center;"  onclick={witnessFileHandler}>
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-person m-2" viewBox="0 0 16 16">
  <path d="M8 8a3 3 0 1 0 0-6 3 3 0 0 0 0 6m2-3a2 2 0 1 1-4 0 2 2 0 0 1 4 0m4 8c0 1-1 1-1 1H3s-1 0-1-1 1-4 6-4 6 3 6 4m-1-.004c-.001-.246-.154-.986-.832-1.664C11.516 10.68 10.289 10 8 10s-3.516.68-4.168 1.332c-.678.678-.83 1.418-.832 1.664z"/>
</svg>
            Witness
        </div>
    )
}

export default WitnessFile