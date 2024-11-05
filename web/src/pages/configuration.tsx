import { useNavigate } from '@solidjs/router';
import { Component, createEffect, createSignal, For } from "solid-js";
import axios from "axios";
import { API_BASE_ENDPOINT } from '../config/constants';
import { setAppState } from '../store/store';

const ConfigsPage: Component = () => {

    const navigate = useNavigate();
    const [chainUsed, setChainUsed] = createSignal<string>("");
    const [fileMode, setFileMode] = createSignal<string>("");
    const [domain, setDomain] = createSignal<string>("");
    const [contract, setContract] = createSignal<string>("0x45f59310ADD88E6d23ca58A0Fa7A55BEE6d2a611");

    createEffect(async () => {
        //fetch conf
        const response = await axios.get(`${API_BASE_ENDPOINT}/explorer_fetch_configuration`, {
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded'
            }
        });

        if (response.status === 200) {
            setChainUsed(response.data.chain)
            setDomain(response.data.domain)
            setFileMode(response.data.mode)
            setContract(response.data.contract)

            setAppState("config", {
                chain: response.data.chain,
                domain: response.data.domain,
                fileMode: response.data.mode,
                contractAddress: response.data.contract,
            })
        }
    })

    const deleteAllFiles = async () => {
        const response = await axios.get(`${API_BASE_ENDPOINT}/explorer_delete_all_files`, {
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded'
            }
        });

        if (response.status === 200) {
            alert("All files deleted")
        }
    }

    const updateConfiguration = async () => {
        const formData = new URLSearchParams();
        formData.append('chain', chainUsed());
        formData.append('domain', domain());
        formData.append('mode', fileMode());
        formData.append('contract', contract());


        const response = await axios.post(`${API_BASE_ENDPOINT}/explorer_update_configuration`, formData, {
            headers: {
                // 'Content-Type': 'application/x-www-form-urlencoded'
            }
        });

        if (response.status === 200) {
            alert("Configuration update success")
        }
    }

    return (
        <>
            <main class="flex-grow p-6">
                <div class="grid lg:grid-cols-4 gap-6">

                    <div class="lg:col-span-6 space-y-6">
                        <div class="card p-6">
                            <div class="flex justify-between items-center mb-4">
                                <p class="card-title">Generel Product Data</p>
                                <div class="inline-flex items-center justify-center rounded-lg bg-slate-100 dark:bg-slate-700 w-9 h-9">
                                    <i class="mgc_transfer_line"></i>
                                </div>
                            </div>

                            <div class="flex flex-col gap-3">
                                <div class="">
                                    <label for="project-name" class="mb-2 block">Domain Name</label>
                                    <input type="email"
                                        id="project-name"
                                        class="form-input"
                                        placeholder="Enter Title"
                                        value={domain()} onChange={(e) => { setDomain(e.target.value) }} aria-describedby="input-helper-text" />
                                </div>

                                <div class="">
                                    <label for="project-name" class="mb-2 block">Contract Address</label>
                                    <input type="email"
                                        id="project-name"
                                        class="form-input"
                                        placeholder="Enter Title"
                                        value={contract()} onChange={(e) => { setContract(e.target.value) }} aria-describedby="input-helper-text" />
                                </div>


                                <div>
                                    <label for="select-label" class="mb-2 block">Selet chain</label>
                                    <select id="select-label" class="form-select"
                                        value={chainUsed()}
                                        onChange={(e) => setChainUsed(e.target.value)}>
                                        <option selected>Open this select menu</option>
                                        <option value="sepolia">Sepolia</option>
                                        <option value="mainnet">Mainnet</option>
                                        <option value="holesky">Holesky</option>
                                    </select>
                                </div>

                                <div>
                                    <label for="select-label" class="mb-2 block">Default File mode (is a file public or private by default)</label>
                                    <select id="select-label" class="form-select"
                                        value={fileMode()}
                                        onChange={(e) => setFileMode(e.target.value)}>
                                        <option>Open this select menu</option>
                                        <option selected={fileMode() === 'public'} value="public">Public</option>
                                        <option selected={fileMode() === 'private'} value="private">private</option>
                                    </select>
                                </div>
                            </div>

                            <div class="lg:col-span-4 mt-5">
                                <div class="flex justify-end gap-3">

                                    <button onClick={(e) => {
                                        e.preventDefault();
                                        updateConfiguration();
                                    }} type="button" class="inline-flex items-center rounded-md border border-transparent bg-green-500 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-green-500 focus:outline-none">
                                        Save
                                    </button>
                                </div>
                            </div>
                        </div>

                        <div class="card p-6">
                            <div class="flex justify-between items-center mb-4">
                                <p class="card-title">Delete all  Data</p>

                                <div class="lg:col-span-4 mt-5">
                                    <div class="flex justify-end gap-3">
                                        <button onClick={(e) => {
                                            deleteAllFiles()
                                        }} type="button" class="inline-flex items-center rounded-md border border-transparent bg-red-500 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-red-500 focus:outline-none">
                                            Delete
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>

                </div>
            </main>
        </>
    );

};

export default ConfigsPage;