import { formatCryptoAddress, remove0xPrefix } from "../util";
import { fetchFiles } from "../network/api";
import { appState, setAppState } from "../store/store";
import { ApiFileInfo } from "../models/FileInfo";
import axios from "axios";
import { API_BASE_ENDPOINT } from "../config/constants";
import { generateNonce, SiweMessage } from "siwe";
import { BrowserProvider } from "ethers";

const SignInButton = () => {

    function createSiweMessage(address: string, statement: string) {

        const scheme = window.location.protocol.slice(0, -1);
        const domain = window.location.host;
        const origin = window.location.origin;
        console.log("Scheme: ", scheme,)
        const message = new SiweMessage({
            // Setting scheme is giving out lots of headaches
            // scheme: scheme,
            domain,
            address,
            statement,
            uri: origin,
            version: '1',
            chainId: 1,
            nonce: generateNonce()
        });
        return message.prepareMessage();
    }

    const signAndConnect = async () => {
        if (window.ethereum) {
            const provider = new BrowserProvider(window.ethereum);
            try {
                // Connect wallet
                const accounts = await window.ethereum.request({ method: 'eth_requestAccounts' });
                const signer = await provider.getSigner();

                // Create a SIWE msg for signing
                const message = createSiweMessage(
                    signer.address,
                    'Sign in with Ethereum to the app.'
                );

                let signature = await signer.signMessage(message);

                const formData = new URLSearchParams();

                formData.append('message', message);
                formData.append('signature', remove0xPrefix(signature));

                console.log("Message for backend: ", message)

                const response = await axios.post(`${API_BASE_ENDPOINT}/siwe`, formData, {
                    headers: {
                        'Content-Type': 'application/x-www-form-urlencoded'
                    }
                });

                if (response.status === 200) {
                    if (signature) {
                        const responseData = response.data
                        const walletAddress = responseData?.address
                        setAppState("metaMaskAddress", walletAddress);

                        let files = await fetchFiles(walletAddress);
                        setAppState('filesFromApi', files);
                    }
                }

            } catch (error: any) {
                console.error('Error during wallet connection or signing:', error);
                alert(error.message);
            }
        } else {
            alert('MetaMask is not installed');
        }
    };

    return (
        <button onClick={(e) => {
            signAndConnect()
        }} data-fc-type="dropdown" data-fc-placement="bottom"
            type="button"
            class="btn rounded-full inline-flex justify-center items-center bg-info text-white w-full">
            <i class="mgc_add_line text-lg me-2"></i> Sign in with MetaMask
        </button>
    )
}


const CustomNavbar = () => {


    const autoConnectWallet = async () => {
        if (window.ethereum) {
            const accounts = await window.ethereum.request({ method: 'eth_requestAccounts' });
            const walletAddress = accounts[0];
            setAppState("metaMaskAddress", walletAddress);

            let files = await fetchFiles(walletAddress);
            setAppState('filesFromApi', files);
        }
    }

    const disconnect = () => {

        let newFiles: Array<ApiFileInfo> = [];

        let files = appState.filesFromApi;
        for (let index = 0; index < files.length; index++) {
            const element = files[index];
            if (element.mode == "public") {
                newFiles.push(element)
            }
        }
        setAppState("filesFromApi", newFiles);
        setAppState("metaMaskAddress", null);
    }

    // createEffect(async () => {
    //     autoConnectWallet()
    // })


    return (
        <header class="app-header flex items-center px-4 gap-3">
            {/* Topbar Brand Logo */}
            <a href="/" style={"max-height: 70px;"}>
                <img
                    src="/images/logo.png"
                    class="logo-sm h-100 img-fluid"
                    alt="Small logo"
                    width={"120px"}
                    height={"70px"}
                    style={"max-height: 100%;"}
                />
            </a>

            <div class="ms-auto">
                {
                    appState.metaMaskAddress == null ?
                        <SignInButton />
                        :
                        <button onclick={disconnect} class="btn rounded-full bg-primary/25 text-primary hover:bg-primary hover:text-white">
                            <label>{formatCryptoAddress(appState.metaMaskAddress)} - Disconnect</label>
                        </button>
                }
            </div>
        </header>

    )
}

export default CustomNavbar