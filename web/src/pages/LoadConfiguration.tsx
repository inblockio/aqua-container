import axios from 'axios';
import { Component, createEffect } from 'solid-js'
import { API_BASE_ENDPOINT } from '../config/constants';
import { setAppState } from '../store/store';

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

    return (
        <></>
    )
}

export default LoadConfiguration