import axios from "axios";
import { ethers } from "ethers";
import { FileInfo } from "../models/PageData";
import { appState, setAppState } from "../store/store";
import { API_BASE_ENDPOINT } from "../config/constants";

interface ISignRevision {
    pageVerificationHash: String
    filename: string
}

const SignFile = ({ pageVerificationHash, filename }: ISignRevision) => {

    const signFileHandler = async () => {
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
                const message = "I sign the following page verification_hash: [0x" + pageVerificationHash + "]"

                // Create an ethers provider
                const provider = new ethers.BrowserProvider(window.ethereum);
                const signer = await provider.getSigner();

                // Hash the message (optional but recommended)
                const messageHash = ethers.hashMessage(message);

                // Sign the message using ethers.js
                const signature = await signer.signMessage(message);
                const signerAddress = await signer.getAddress()

                console.log("Signature: ", signature)

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
                        // const publicKey = ethers.recoverAddress(messageHash, signature)
                        const publicKey = ethers.SigningKey.recoverPublicKey(
                            messageHash,
                            signature,
                        )

                        console.log("Public key", publicKey)

                        const formData = new URLSearchParams();
                        formData.append('filename', filename);
                        formData.append('signature', signature);
                        /* Recovered public key if needed */
                        // formData.append('publickey', walletAddress);
                        /* Hardcoded public key value for now; Remove this once a fix for obtaining public keys is found */
                        // formData.append('publickey', "0x04c56c1231c8a69a375c3f81e549413eb0f415cfd56d40c9a5622456a3f77be0625e1fe8a50cb6274e5d0959625bf33f3c8d1606b5782064bad2e4b46c5e2a7428");
                        formData.append('publickey', publicKey)
                        formData.append('wallet_address', ethers.getAddress(walletAddress));

                        const response = await axios.post(`${API_BASE_ENDPOINT}/explorer_sign_revision`, formData, {
                            headers: {
                                'Content-Type': 'application/x-www-form-urlencoded'
                            }
                        });

                        let res = await response.data;
                        let logs: Array<string> = res.logs
                        logs.forEach((item) => {
                            console.log("**>" + item + "\n.")
                        })

                        if (response.status === 200) {
                            let resp: FileInfo = res.file
                            console.log(resp)
                            let array: FileInfo[] = [];
                            for (let index = 0; index < appState.filesFromApi.length; index++) {
                                const element = appState.filesFromApi[index];
                                if (element.name === resp.name) {
                                    array.push(resp)
                                } else {
                                    array.push(element)
                                }
                            }
                            setAppState("filesFromApi", array)
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
        <div class="rounded bg-info/25 px-2 mx-4 " style="display: inline-flex; align-items: center;" onclick={signFileHandler}>
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-pen-fill m-2" viewBox="0 0 16 16">
                <path d="m13.498.795.149-.149a1.207 1.207 0 1 1 1.707 1.708l-.149.148a1.5 1.5 0 0 1-.059 2.059L4.854 14.854a.5.5 0 0 1-.233.131l-4 1a.5.5 0 0 1-.606-.606l1-4a.5.5 0 0 1 .131-.232l9.642-9.642a.5.5 0 0 0-.642.056L6.854 4.854a.5.5 0 1 1-.708-.708L9.44.854A1.5 1.5 0 0 1 11.5.796a1.5 1.5 0 0 1 1.998-.001" />
            </svg>
            Add Signature
        </div>
    )
}

export default SignFile
