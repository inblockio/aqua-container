import { A } from "@solidjs/router";
import { createEffect, createSignal } from "solid-js";
import { formatCryptoAddress } from "../util";
import { fetchFiles } from "../network/api";
import { appState, setAppState } from "../store/store";


const CustomNavbar = () => {

    const signAndConnect = async () => {
        if (window.ethereum) {
            try {
                // Connect wallet
                const accounts = await window.ethereum.request({ method: 'eth_requestAccounts' });
                const walletAddress = accounts[0];

                console.log('Connected account:', walletAddress);

                // Message to sign
                const message = `Please sign this message to prove ownership of the wallet: ${walletAddress}`;

                // Sign the message
                const signature = await window.ethereum.request({
                    method: 'personal_sign',
                    params: [message, walletAddress],
                });

                if (signature) {
                    setAppState("metaMaskAddress",walletAddress);
                }

            } catch (error : any) {
                console.error('Error during wallet connection or signing:', error);
                alert(error.message);
            }
        } else {
            alert('MetaMask is not installed');
        }
    };


    const autoConnectWallet = async () => {
        if (window.ethereum) {
            const accounts = await window.ethereum.request({ method: 'eth_requestAccounts' });
            const walletAddress = accounts[0];
            setAppState("metaMaskAddress",walletAddress);

            let files = await fetchFiles(walletAddress);
            setAppState('filesFromApi', files);
        }

    }

    const disconnect = () => {

        setAppState("metaMaskAddress",null)
    }

    createEffect(async () => {
        autoConnectWallet()
    })


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
                        <button onClick={(e) => {
                            signAndConnect()
                        }} data-fc-type="dropdown" data-fc-placement="bottom"
                            type="button"
                            class="btn rounded-full inline-flex justify-center items-center bg-info text-white w-full">
                            <i class="mgc_add_line text-lg me-2"></i> Sign in with MetaMask
                        </button> :
                        <button onclick={disconnect} class="btn rounded-full bg-primary/25 text-primary hover:bg-primary hover:text-white">
                            <label>{formatCryptoAddress(appState.metaMaskAddress)} - Disconnect</label>
                        </button>
                }
            </div>
        </header>

    )
}

export default CustomNavbar