import axios from "axios";
import { useEffect } from "react";
import { fetchFiles, getCookie } from "../utils/functions";
import { ENDPOINTS } from "../utils/constants";
import { useStore } from "zustand";
import appStore from "../store";


const LoadConfiguration = () => {
    const { setMetamaskAddress, setConfiguration, setFiles } = useStore(appStore)
    const fetchAddressGivenANonce = async (nonce: string) => {
        const formData = new URLSearchParams();
        formData.append('nonce', nonce);

        const response = await axios.post(ENDPOINTS.FETCH_ADDRESS_BY_NONCE, formData, {
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded'
            }
        });
        if (response.status === 200) {
            const address = response.data?.address
            if (address) {
                setMetamaskAddress(address)
                let files = await fetchFiles(address);
                setFiles(files)
            }
        }
    }

    const fetchConfiguration = async () => {

        const response = await axios.get(ENDPOINTS.FETCH_CONFIGURATION, {
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded'
            }
        });

        if (response.status === 200) {
            setConfiguration({
                network: response.data.chain,
                domain: response.data.domain,
                fileMode: response.data.mode,
                contractAddress: response.data.contract,
            })
        }
    }



    useEffect(() => {
        const nonce = getCookie("pkc_nonce");
        if (nonce) {
            fetchAddressGivenANonce(nonce)
        } else {
            alert("Your session has expired. Sign in again!")
        }
    }, []);

    useEffect(() => {
        //fetch configuration
        fetchConfiguration()
    }, [])

    return (
        <></>
    )
}

export default LoadConfiguration