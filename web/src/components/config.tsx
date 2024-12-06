import axios from "axios";
import { useEffect } from "react";
import { fetchFiles, generateAvatar, getCookie } from "../utils/functions";
import { ENDPOINTS } from "../utils/constants";
import { useStore } from "zustand";
import appStore from "../store";
import { toaster } from "./ui/toaster";
import { ethers } from "ethers";


const LoadConfiguration = () => {
    const { setMetamaskAddress, setUserProfile, setFiles, setAvatar } = useStore(appStore)

    const fetchAddressGivenANonce = async (nonce: string) => {
        try {
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
                    fetchUserProfile(_address)
                }
            } else {
                setMetamaskAddress(null)
                setAvatar(undefined)
                setFiles([])
                setUserProfile({
                    network: '',
                    domain: '',
                    fileMode: '',
                    contractAddress: '',
                })
            }
        }
        catch (error: any) {
            if (error?.response?.status === 404) {
                setMetamaskAddress(null)
                setAvatar(undefined)
                setFiles([])
            } else {

            }
        }
    }

    const fetchUserProfile = async (address: string) => {

        const response = await axios.get(ENDPOINTS.FETCH_USER_PROFILE, {
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded',
                'metamask_address': address
            }
        });

        if (response.status === 200) {
            setUserProfile({
                network: response.data.user_profile.chain,
                domain: response.data.user_profile.domain,
                fileMode: response.data.user_profile.mode,
                contractAddress: response.data.user_profile.contract,
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

    // useEffect(() => {
    //     //fetch user profile
    //     fetchUserProfile()
    // }, [metamaskAddress])

    return (
        <></>
    )
}

export default LoadConfiguration