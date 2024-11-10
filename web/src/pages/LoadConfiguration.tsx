import axios from 'axios';
import { Component, createEffect } from 'solid-js'
import { API_BASE_ENDPOINT } from '../config/constants';
import { appState, setAppState } from '../store/store';
import { getCookie } from '../util';
import { fetchFiles } from '../network/api';

const LoadConfiguration: Component = () => {

    createEffect(async () => {
        //fetch conf
        const response = await axios.get(`${API_BASE_ENDPOINT}/explorer_fetch_configuration`, {
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded'
            }
        });

        if (response.status === 200) {
            setAppState("config", {
                chain: response.data.chain,
                domain: response.data.domain,
                fileMode: response.data.mode,
                contractAddress: response.data.contract,
            })
        }
    })

    const fetchAddressGivenANonce = async (nonce: string) => {
        const formData = new URLSearchParams();
        formData.append('nonce', nonce);

        const response = await axios.post(`${API_BASE_ENDPOINT}/fetch_nonce_session`, formData, {
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded'
            }
        });
        if (response.status === 200) {
            const address = response.data?.address
            if(address){
                setAppState('metaMaskAddress', address)
                let files = await fetchFiles(address);
                setAppState('filesFromApi', files);
            }
        }
    }



    createEffect(() => {
        const nonce = getCookie("pkc_nonce");
        if (nonce) {
            fetchAddressGivenANonce(nonce)
        } else {
            alert("Your session has expired. Sign in again!")
        }
    });

    return (
        <></>
    )
}

export default LoadConfiguration