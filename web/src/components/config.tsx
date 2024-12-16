import axios from "axios";
import { useEffect } from "react";
import { fetchFiles, generateAvatar, getCookie } from "../utils/functions";
import { useStore } from "zustand";
import appStore from "../store";
import { toaster } from "./ui/toaster";
import { ethers } from "ethers";


const LoadConfiguration = () => {
    const { setMetamaskAddress, setUserProfile, setFiles, setAvatar, backend_url } = useStore(appStore)

    const fetchAddressGivenANonce = async (nonce: string) => {
        if (!backend_url.includes('0.0.0.0')) {
            try {
                const formData = new URLSearchParams();
                formData.append('nonce', nonce);

                const url = `${backend_url}/fetch_nonce_session`;
                console.log("url is ", url)
                const response = await axios.post(url, formData, {
                    headers: {
                        'Content-Type': 'application/x-www-form-urlencoded'
                    }
                });

                if (response.status === 200) {
                    const url2 = `${backend_url}/explorer_files`;
                    const _address = response.data?.address
                    if (_address) {
                        const address = ethers.getAddress(_address)
                        setMetamaskAddress(address)
                        const avatar = generateAvatar(address)
                        setAvatar(avatar)
                        const files = await fetchFiles(address, url2);
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
                    console.log("An error from the api ", error);
                }
            }
        }
    }

    const fetchUserProfile = async (address: string) => {

        const url = `${backend_url}/explorer_fetch_user_profile`;
        console.log("url is ", url);

        const response = await axios.get(url, {
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
        if (!backend_url.includes("0.0.0.0")) {
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
        }
    }, [backend_url]);

    // useEffect(() => {
    //     //fetch user profile
    //     fetchUserProfile()
    // }, [metamaskAddress])

    return (
        <></>
    )
}

export default LoadConfiguration