import axios from "axios";
import { useEffect } from "react";
import { fetchFiles, generateAvatar, getCookie } from "../utils/functions";
import { ENDPOINTS } from "../utils/constants";
import { useStore } from "zustand";
import appStore from "../store";
import { toaster } from "./ui/toaster";
import { ethers } from "ethers";


const LoadConfiguration = () => {
    const { setMetamaskAddress, setConfiguration, setFiles, setAvatar } = useStore(appStore)
    const fetchAddressGivenANonce = async (nonce: string) => {
        const formData = new URLSearchParams();
        formData.append('nonce', nonce);

        const response = await axios.post(ENDPOINTS.FETCH_ADDRESS_BY_NONCE, formData, {
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded'
            }
        });
        if (response.status === 200) {
            const _address = response.data?.address
            if (_address) {
                let address = ethers.getAddress(_address)
                setMetamaskAddress(address)
                let avatar = generateAvatar(address)
                setAvatar(avatar)
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
            setMetamaskAddress(null)
            setAvatar(undefined)
            toaster.create({
                description: "You are not logged in! Please login",
                type: "info",
            })
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