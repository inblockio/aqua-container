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
        <button class="btn bg:green" onclick={witnessFileHandler}>
            Witness
        </button>
    )
}

export default WitnessFile