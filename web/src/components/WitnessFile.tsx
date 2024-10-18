import axios from "axios";
import { ethers } from "ethers";
import { ETH_CHAINID_MAP, SEPOLIA_SMART_CONTRACT_ADDRESS } from "../config/constants";
import sha3 from 'js-sha3'
import { FileInfo } from "../models/FileInfo";
import { appState, setAppState } from "../store/store";

async function storeWitnessTx(filename: string, txhash: string, ownerAddress: string) {

    const formData = new URLSearchParams();

    formData.append('filename', filename);
    formData.append('tx_hash', txhash);
    formData.append('wallet_address', ownerAddress);



    let response = await axios.post("http://localhost:3600/explorer_witness_file", formData, {
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded'
        }
    })

    let res = await response.data ;
    let logs : Array<string> = res.logs
    logs.forEach((item)=>{
        console.log("**>" + item + "\n." )
    })
   
    if (response.status === 200) {
        let resp: FileInfo = res.file as FileInfo 
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
        alert("Witnessed successfully")
    }

}

interface IWitnessFile {
    previousVerificationHash?: String | null
    filename: string
}

const WitnessFile = ({ previousVerificationHash, filename }: IWitnessFile) => {

    const witnessFileHandler = async () => {
        console.log("Previous verification hash: ", previousVerificationHash)
        const witness_event_verification_hash = sha3.sha3_512("a69f73cca23a9ac5c8b567dc185a756e97c982164fe25859e0d1dcc1475c80a615b2123af1f5f94c11e3e9402c3ac558f500199d95b6d3e301758586281dcd26" + previousVerificationHash)
        console.log("Witness event verification hash: ", witness_event_verification_hash)
        if (window.ethereum) {
            try {
                // Connect wallet
                const accounts = await window?.ethereum?.request({ method: 'eth_requestAccounts' });
                const walletAddress = accounts[0];

                if (!walletAddress) {
                    alert("Please connect your wallet to continue");
                    return;
                }

                const chainId = await window.ethereum.request({ method: 'eth_chainId' })
                // const serverChainId = ethChainIdMap[parsed.witness_network]
                // if (serverChainId !== chainId) {
                if (ETH_CHAINID_MAP.sepolia !== chainId) {
                    console.log(ETH_CHAINID_MAP.sepolia, chainId)
                    // Switch network if the Wallet network does not match DA
                    // server network.
                    await window.ethereum.request({
                        method: 'wallet_switchEthereumChain',
                        params: [{
                            chainId: ETH_CHAINID_MAP.sepolia,
                        }],
                    })
                }
                const params = [
                    {
                        from: walletAddress,
                        to: SEPOLIA_SMART_CONTRACT_ADDRESS,
                        // gas and gasPrice are optional values which are
                        // automatically set by MetaMask.
                        // gas: '0x7cc0', // 30400
                        // gasPrice: '0x328400000',
                        data: '0x9cef4ea1' + witness_event_verification_hash,
                    },
                ]
                window.ethereum
                    .request({
                        method: 'eth_sendTransaction',
                        params: params,
                    })
                    .then(txhash => {
                        console.log("Transaction hash is: ", txhash)
                        storeWitnessTx(filename, txhash, ethers.getAddress(walletAddress)).then(() => {
                            console.log("State updated successfully")
                        }).catch(() => { alert("Something went wrong") })
                    })

            } catch (error) {
                console.error('Error during wallet connection or signing:', error);
            }
        } else {
            alert('MetaMask is not installed');
        }
    };

    return (
        <div class="rounded bg-primary/25 px-2 mx-4" style="cursor: pointer; display: inline-flex; align-items: center;" onclick={witnessFileHandler}>
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-person m-2" viewBox="0 0 16 16">
                <path d="M8 8a3 3 0 1 0 0-6 3 3 0 0 0 0 6m2-3a2 2 0 1 1-4 0 2 2 0 0 1 4 0m4 8c0 1-1 1-1 1H3s-1 0-1-1 1-4 6-4 6 3 6 4m-1-.004c-.001-.246-.154-.986-.832-1.664C11.516 10.68 10.289 10 8 10s-3.516.68-4.168 1.332c-.678.678-.83 1.418-.832 1.664z" />
            </svg>
            Witness
        </div>
    )
}

export default WitnessFile