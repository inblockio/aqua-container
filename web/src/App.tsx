import type {Component} from 'solid-js';
import {createEffect, createSignal} from "solid-js";
import axios from "axios";
import { ethers } from "ethers";
import {FileInfo} from "./models/FileInfo";

const App: Component = () => {
    const [metaMaskAddress, setMetaMaskAddress] = createSignal<string | null>(null);
    const [selectedFileForUpload, setSelectedFileForUpload] = createSignal<File | null>(null);
    const [fileFromApi, setFilesFromApi] = createSignal<Array<FileInfo>>([]);
    const [error, setError] = createSignal<string>('');
    const maxFileSize = 20 * 1024 * 1024; // 20 MB in bytes

    let fileInput;

    createEffect(async () => {

        let files  = await fetchFiles();
        setFilesFromApi(files);

    })

    // Check if MetaMask is installed
    const isMetaMaskInstalled = () => {
        return typeof window.ethereum !== "undefined";
    };

    // Request access to MetaMask
    const connectToMetaMask = async () => {


        if (!isMetaMaskInstalled()) {
            alert("MetaMask is not installed. Please install MetaMask and try again.");
            return;
        }

        try {
            // Create a new Web3Provider using BrowserProvider in v6
            const provider = new ethers.BrowserProvider(window.ethereum);

            // Request MetaMask accounts
            const accounts = await provider.send("eth_requestAccounts", []);

            // Get the signer
            const signer = await provider.getSigner();

            // Get the address
            const address = await signer.getAddress();

            setMetaMaskAddress(address); // Set the address
        } catch (error) {
            console.error("Error connecting to MetaMask:", error);
        }
    };


    const handleSelectFileForUploadClick = () => {
        // Trigger the hidden file input click
        fileInput.click();
    };
    const handleFileSelect = (event: Event) => {
        const input = event.target as HTMLInputElement;
        const file = input.files?.[0];

        if (!file) {
            setSelectedFileForUpload(null);
            setError('');
            return;
        }

        if (file.size > maxFileSize) {
            setSelectedFileForUpload(null);
            setError('File size exceeds 20 MB limit');
            // Reset the input
            input.value = '';
            return;
        }

        setSelectedFileForUpload(file);
        setError('');


        //upload the file
        uploadFile()
    };

    const uploadFile = async () => {
        const file = selectedFileForUpload();
        if (!file) {
            setError('No file selected');
            return;
        }

        const formData = new FormData();
        formData.append('file', file);
        formData.append('account', "example");

        try {
            const response = await axios.post('http://127.0.0.1:3600/explorer_file_upload', formData, {
                headers: {
                    'Content-Type': 'multipart/form-data',
                },
            });
            console.log('File uploaded successfully:', response.data);
        } catch (error) {
            console.error('Error uploading file:', error);
            setError('Failed to upload file');
        }
    };

    async function fetchFiles(): Promise<Array<FileInfo>> {
        try {
            const response = await fetch("http://127.0.0.1:3600/explorer_files");

            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }

            console.log("fetchFiles Response "+ response.ok)

            // Parse the response body as JSON
            const data = await response.json();

            // Assuming the API returns an array of FileInfo objects
            const files: Array<FileInfo> = data.map((item: any) => ({
                id: item.id,
                name: item.name,
                file_type: item.file_type,
                size: item.size,
                page_data: item.page_data
            }));

            return files;
        } catch (error) {
            console.error("Error fetching files:", error);
            return [];
        }
    }
    return (
        <div class="flex wrapper">


            {/*   <!-- ==============================================================  --> */}
            {/*   <!-- Start Page Content here  --> */}
            {/*   <!-- ==============================================================  --> */}

            <div class="page-content">


                <main class="flex-grow p-6">


                    <div class="flex">
                        <div id="default-offcanvas"
                             class="lg:block hidden top-0 left-0 transform h-full min-w-[16rem] me-6 card rounded-none lg:rounded-md fc-offcanvas-open:translate-x-0 lg:z-0 z-50 fixed lg:static lg:translate-x-0 -translate-x-full transition-all duration-300"
                             tabindex="-1">
                            <div class="p-5">
                                <div class="relative">

                                    <a href="javascript:void(0)" onClick={(e) => {
                                        handleSelectFileForUploadClick()
                                    }} data-fc-type="dropdown" data-fc-placement="bottom"
                                       type="button"
                                       class="btn inline-flex justify-center items-center bg-primary text-white w-full mb-3">
                                        <i class="mgc_add_line text-lg me-2"></i> Upload File
                                    </a>
                                    <br/>

                                    <a href="javascript:void(0)" onClick={(e) => {
                                        console.log("todo")
                                    }} data-fc-type="dropdown" data-fc-placement="bottom"
                                       type="button"
                                       class="btn inline-flex justify-center items-center bg-primary text-white w-full mt-4">
                                        <i class="mgc_add_line text-lg me-2"></i> Verify aqua file
                                    </a>
                                    {/* Hidden file input */}
                                    <input
                                        type="file"
                                        ref={el => fileInput = el} // Save reference to the input element
                                        style={{display: "none"}} // Hide the input element
                                        onChange={handleFileSelect}
                                    />

                                </div>

                                <div class="mt-6">
                                    <h6 class="text-uppercase mt-3">Storage</h6>
                                    <div
                                        class="flex w-full h-1.5 bg-gray-200 rounded-full overflow-hidden dark:bg-gray-700 mt-4">
                                        <div class="flex flex-col justify-center overflow-hidden bg-primary"
                                             role="progressbar" style="width: 46%" aria-valuenow="46" aria-valuemin="0"
                                             aria-valuemax="100"></div>
                                    </div>
                                    <p class="text-gray-500 mt-4 text-xs">70 files (15 GB)</p>
                                </div>
                            </div>
                        </div>

                        <div class="w-full">
                            <div class="grid 2xl:grid-cols-4 sm:grid-cols-2 grid-cols-1 gap-6">
                                <div class="2xl:col-span-4 sm:col-span-2">
                                    {
                                        error().length == 0 ? <div></div> :
                                            <>


                                                <div id="dismiss-alert"
                                                     class="bg-red-500 text-sm text-white transition duration-300 bg-teal-50 border border-teal-200 rounded-md p-4"
                                                     role="alert">
                                                    <div class="flex items-center gap-3">
                                                        <div class="flex-shrink-0">
                                                            <i class="mgc_-badge-check text-xl"></i>
                                                        </div>
                                                        <div class="flex-grow">
                                                            <div class="text-sm text-teal-800 font-medium">
                                                                {error()}

                                                            </div>
                                                        </div>
                                                        <button onClick={(e) => {
                                                            setError("");
                                                            setSelectedFileForUpload(null)
                                                        }} data-fc-dismiss="dismiss-alert" type="button"
                                                                id="dismiss-test"
                                                                class="ms-auto h-8 w-8 rounded-full bg-gray-200 flex justify-center items-center">
                                                            {/*<i class="mgc_close_line text-xl"></i>*/}
                                                            <svg xmlns="http://www.w3.org/2000/svg" width="16"
                                                                 height="16" fill="currentColor" class="bi bi-x"
                                                                 viewBox="0 0 16 16">
                                                                <path
                                                                    d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708"/>
                                                            </svg>
                                                        </button>
                                                    </div>
                                                </div>
                                                <br/>
                                            </>
                                    }
                                    <div class="flex items-center justify-between gap-4">
                                        <div class="lg:hidden block">
                                            <button data-fc-target="default-offcanvas" data-fc-type="offcanvas"
                                                    class="inline-flex items-center justify-center text-gray-700 border border-gray-300 rounded shadow hover:bg-slate-100 dark:text-gray-400 hover:dark:bg-gray-800 dark:border-gray-700 transition h-9 w-9 duration-100">
                                                <div class="mgc_menu_line text-lg"></div>
                                            </button>
                                        </div>


                                        <h4 class="text-xl">Folders</h4>

                                        <form class="ms-auto">
                                            {/*<div class="flex items-center">*/}
                                            {/*    <input type="text" class="form-input  rounded-full"*/}
                                            {/*           placeholder="Search files..."/>*/}
                                            {/*    <span class="mgc_search_line text-xl -ms-8"></span>*/}
                                            {/*</div>*/}

                                            {
                                                metaMaskAddress() == null ? <a href="javascript:void(0)" onClick={(e) => {
                                                    connectToMetaMask()
                                                }} data-fc-type="dropdown" data-fc-placement="bottom"
                                                                               type="button"
                                                                               class="btn inline-flex justify-center items-center bg-info text-white w-full mt-4">
                                                    <i class="mgc_add_line text-lg me-2"></i> sign in with metamask
                                                </a> : <label>{metaMaskAddress()}</label>
                                            }

                                        </form>
                                    </div>
                                </div>

                                <div class="card">
                                    <div class="p-5">
                                        <div class="space-y-4 text-gray-600 dark:text-gray-300">
                                            <div class="flex items-start relative gap-5">
                                                <div class="flex items-center gap-3">
                                                    <div class="h-14 w-14">
                                                   <span class="flex h-full w-full items-center justify-center">
                                                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24"
                                                             viewBox="0 0 24 24" fill="none" stroke="currentColor"
                                                             stroke-width="2" stroke-linecap="round"
                                                             stroke-linejoin="round"
                                                             class="feather feather-folder h-full w-full fill-warning text-warning"><path
                                                            d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path></svg>
                                                    </span>
                                                    </div>
                                                    <div class="space-y-1">
                                                        <p class="font-semibold text-base">Document</p>
                                                        <p class="text-xs">Using 25% of storage</p>
                                                    </div>
                                                </div>
                                                <div class="flex items-center absolute top-0 end-0">
                                                    <button data-fc-type="dropdown" data-fc-placement="bottom-end"
                                                            class="inline-flex text-slate-700 hover:bg-slate-100 dark:hover:bg-gray-700 dark:text-gray-300 rounded-full p-2">
                                                        <i data-feather="more-vertical" class="w-4 h-4"></i>
                                                    </button>

                                                    <div
                                                        class="fc-dropdown hidden fc-dropdown-open:opacity-100 opacity-0 w-40 z-50 mt-2 transition-all duration-300 bg-white dark:bg-gray-800 shadow-lg border border-gray-200 dark:border-gray-700 rounded-lg p-2">
                                                        <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                           href="apps-file-manager.html#">
                                                            <i data-feather="edit-3" class="w-4 h-4 me-3"></i>
                                                            Edit
                                                        </a>
                                                        <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                           href="apps-file-manager.html#">
                                                            <i data-feather="link" class="w-4 h-4 me-3"></i>
                                                            Copy Link
                                                        </a>
                                                        <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                           href="apps-file-manager.html#">
                                                            <i data-feather="share-2" class="w-4 h-4 me-3"></i>
                                                            Share
                                                        </a>
                                                        <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                           href="apps-file-manager.html#">
                                                            <i data-feather="download" class="w-4 h-4 me-3"></i>
                                                            Download
                                                        </a>
                                                    </div>
                                                </div>
                                            </div>
                                            <div class="flex items-center gap-2">
                                                <p class="text-sm">3 GB</p>
                                                <span class="p-0.5 bg-gray-600 rounded-full"></span>
                                                <p class="text-sm">400 Files</p>
                                            </div>
                                        </div>
                                    </div>
                                    {/*   <!-- end card body  --> */}
                                </div>

                                <div class="card">
                                    <div class="p-5">
                                        <div class="space-y-4 text-gray-600 dark:text-gray-300">
                                            <div class="flex items-start relative gap-5">
                                                <div class="flex items-center gap-3">
                                                    <div class="h-14 w-14">
                                                    <span class="flex h-full w-full items-center justify-center">
                                                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24"
                                                             viewBox="0 0 24 24" fill="none" stroke="currentColor"
                                                             stroke-width="2" stroke-linecap="round"
                                                             stroke-linejoin="round"
                                                             class="feather feather-folder h-full w-full fill-warning text-warning"><path
                                                            d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path></svg>
                                                    </span>
                                                    </div>
                                                    <div class="space-y-1">
                                                        <p class="font-semibold text-base">Music</p>
                                                        <p class="text-xs">Using 16% of storage</p>
                                                    </div>
                                                </div>
                                                <div class="flex items-center absolute top-0 end-0">
                                                    <button data-fc-type="dropdown" data-fc-placement="bottom-end"
                                                            class="inline-flex text-slate-700 hover:bg-slate-100 dark:hover:bg-gray-700 dark:text-gray-300 rounded-full p-2">
                                                        <i data-feather="more-vertical" class="w-4 h-4"></i>
                                                    </button>

                                                    <div
                                                        class="fc-dropdown hidden fc-dropdown-open:opacity-100 opacity-0 w-40 z-50 mt-2 transition-all duration-300 bg-white dark:bg-gray-800 shadow-lg border border-gray-200 dark:border-gray-700 rounded-lg p-2">
                                                        <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                           href="apps-file-manager.html#">
                                                            <i data-feather="edit-3" class="w-4 h-4 me-3"></i>
                                                            Edit
                                                        </a>
                                                        <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                           href="apps-file-manager.html#">
                                                            <i data-feather="link" class="w-4 h-4 me-3"></i>
                                                            Copy Link
                                                        </a>
                                                        <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                           href="apps-file-manager.html#">
                                                            <i data-feather="share-2" class="w-4 h-4 me-3"></i>
                                                            Share
                                                        </a>
                                                        <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                           href="apps-file-manager.html#">
                                                            <i data-feather="download" class="w-4 h-4 me-3"></i>
                                                            Download
                                                        </a>
                                                    </div>
                                                </div>
                                            </div>
                                            <div class="flex items-center gap-2">
                                                <p class="text-sm">1.5 GB</p>
                                                <span class="p-0.5 bg-gray-600 rounded-full"></span>
                                                <p class="text-sm">212 Files</p>
                                            </div>
                                        </div>
                                    </div>
                                    {/*   <!-- end card body  --> */}
                                </div>

                                <div class="card">
                                    <div class="p-5">
                                        <div class="space-y-4 text-gray-600 dark:text-gray-300">
                                            <div class="flex items-start relative gap-5">
                                                <div class="flex items-center gap-3">
                                                    <div class="h-14 w-14">
                                                    <span class="flex h-full w-full items-center justify-center">
                                                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24"
                                                             viewBox="0 0 24 24" fill="none" stroke="currentColor"
                                                             stroke-width="2" stroke-linecap="round"
                                                             stroke-linejoin="round"
                                                             class="feather feather-folder h-full w-full fill-warning text-warning"><path
                                                            d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path></svg>
                                                    </span>
                                                    </div>
                                                    <div class="space-y-1">
                                                        <p class="font-semibold text-base">Images</p>
                                                        <p class="text-xs">Using 50% of storage</p>
                                                    </div>
                                                </div>
                                                <div class="flex items-center absolute top-0 end-0">
                                                    <button data-fc-type="dropdown" data-fc-placement="bottom-end"
                                                            class="inline-flex text-slate-700 hover:bg-slate-100 dark:hover:bg-gray-700 dark:text-gray-300 rounded-full p-2">
                                                        <i data-feather="more-vertical" class="w-4 h-4"></i>
                                                    </button>

                                                    <div
                                                        class="fc-dropdown hidden fc-dropdown-open:opacity-100 opacity-0 w-40 z-50 mt-2 transition-all duration-300 bg-white dark:bg-gray-800 shadow-lg border border-gray-200 dark:border-gray-700 rounded-lg p-2">
                                                        <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                           href="apps-file-manager.html#">
                                                            <i data-feather="edit-3" class="w-4 h-4 me-3"></i>
                                                            Edit
                                                        </a>
                                                        <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                           href="apps-file-manager.html#">
                                                            <i data-feather="link" class="w-4 h-4 me-3"></i>
                                                            Copy Link
                                                        </a>
                                                        <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                           href="apps-file-manager.html#">
                                                            <i data-feather="share-2" class="w-4 h-4 me-3"></i>
                                                            Share
                                                        </a>
                                                        <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                           href="apps-file-manager.html#">
                                                            <i data-feather="download" class="w-4 h-4 me-3"></i>
                                                            Download
                                                        </a>
                                                    </div>
                                                </div>
                                            </div>
                                            <div class="flex items-center gap-2">
                                                <p class="text-sm">39 GB</p>
                                                <span class="p-0.5 bg-gray-600 rounded-full"></span>
                                                <p class="text-sm">25 Apps</p>
                                            </div>
                                        </div>
                                    </div>
                                    {/*   <!-- end card body  --> */}
                                </div>

                                <div class="card">
                                    <div class="p-5">
                                        <div class="space-y-4 text-gray-600 dark:text-gray-300">
                                            <div class="flex items-start relative gap-5">
                                                <div class="flex items-center gap-3">
                                                    <div class="h-14 w-14">
                                                    <span class="flex h-full w-full items-center justify-center">
                                                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24"
                                                             viewBox="0 0 24 24" fill="none" stroke="currentColor"
                                                             stroke-width="2" stroke-linecap="round"
                                                             stroke-linejoin="round"
                                                             class="feather feather-folder h-full w-full fill-warning text-warning"><path
                                                            d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path></svg>
                                                    </span>
                                                    </div>
                                                    <div class="space-y-1">
                                                        <p class="font-semibold text-base">Videos</p>
                                                        <p class="text-xs">Using 8% of storage</p>
                                                    </div>
                                                </div>
                                                <div class="flex items-center absolute top-0 end-0">
                                                    <button data-fc-type="dropdown" data-fc-placement="bottom-end"
                                                            class="inline-flex text-slate-700 hover:bg-slate-100 dark:hover:bg-gray-700 dark:text-gray-300 rounded-full p-2">
                                                        <i data-feather="more-vertical" class="w-4 h-4"></i>
                                                    </button>

                                                    <div
                                                        class="fc-dropdown hidden fc-dropdown-open:opacity-100 opacity-0 w-40 z-50 mt-2 transition-all duration-300 bg-white dark:bg-gray-800 shadow-lg border border-gray-200 dark:border-gray-700 rounded-lg p-2">
                                                        <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                           href="apps-file-manager.html#">
                                                            <i data-feather="edit-3" class="w-4 h-4 me-3"></i>
                                                            Edit
                                                        </a>
                                                        <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                           href="apps-file-manager.html#">
                                                            <i data-feather="link" class="w-4 h-4 me-3"></i>
                                                            Copy Link
                                                        </a>
                                                        <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                           href="apps-file-manager.html#">
                                                            <i data-feather="share-2" class="w-4 h-4 me-3"></i>
                                                            Share
                                                        </a>
                                                        <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                           href="apps-file-manager.html#">
                                                            <i data-feather="download" class="w-4 h-4 me-3"></i>
                                                            Download
                                                        </a>
                                                    </div>
                                                </div>
                                            </div>
                                            <div class="flex items-center gap-2">
                                                <p class="text-sm">4 GB</p>
                                                <span class="p-0.5 bg-gray-600 rounded-full"></span>
                                                <p class="text-sm">9 Videos</p>
                                            </div>
                                        </div>
                                    </div>
                                    {/*   <!-- end card body  --> */}
                                </div>

                                <div class="2xl:col-span-4 sm:col-span-2">
                                    <div class="card">
                                        <div class="card-header">
                                            <h4 class="text-lg font-semibold text-gray-800 dark:text-gray-300">Recent
                                                Files</h4>
                                        </div>

                                        <div class="flex flex-col">
                                            <div class="overflow-x-auto">
                                                <div class="inline-block min-w-full align-middle">
                                                    <div class="overflow-hidden">
                                                        <table
                                                            class="min-w-full divide-y divide-gray-200 dark:divide-gray-600">
                                                            <thead class="bg-gray-50 dark:bg-gray-700">
                                                            <tr class="text-gray-800 dark:text-gray-300">
                                                                <th scope="col"
                                                                    class="p-3.5 text-sm text-start font-semibold min-w-[10rem]">File
                                                                    Name
                                                                </th>
                                                                <th scope="col"
                                                                    class="p-3.5 text-sm text-start font-semibold min-w-[10rem]">Last
                                                                    Modified
                                                                </th>
                                                                <th scope="col"
                                                                    class="p-3.5 text-sm text-start font-semibold min-w-[6rem]">File
                                                                    Size
                                                                </th>
                                                                <th scope="col"
                                                                    class="p-3.5 text-sm text-start font-semibold min-w-[8rem]">Owner
                                                                </th>
                                                                <th scope="col"
                                                                    class="p-3.5 text-sm text-start font-semibold min-w-[6rem]">Members
                                                                </th>
                                                                <th scope="col"
                                                                    class="p-3.5 text-sm text-start font-semibold">Action
                                                                </th>
                                                            </tr>
                                                            </thead>
                                                            <tbody
                                                                class="divide-y divide-gray-200 dark:divide-gray-600">
                                                            <tr>
                                                                <td class="p-3.5 text-sm text-gray-700 dark:text-gray-400">
                                                                    <a href="javascript: void(0);" class="font-medium">App
                                                                        Design & Development</a>
                                                                </td>
                                                                <td class="p-3.5 text-sm text-gray-700 dark:text-gray-400">
                                                                    <p>Jan 03, 2020</p>
                                                                    <span class="text-xs">by Andrew</span>
                                                                </td>
                                                                <td class="p-3.5 text-sm text-gray-700 dark:text-gray-400">128
                                                                    MB
                                                                </td>
                                                                <td class="p-3.5 text-sm text-gray-700 dark:text-gray-400">
                                                                    Danielle Thompson
                                                                </td>
                                                                <td class="p-3.5">
                                                                    <div class="flex -space-x-1.5">
                                                                        <img
                                                                            class="inline-block h-6 w-6 rounded-full ring-2 ring-white dark:ring-gray-700"
                                                                            src="images/users/avatar-1.jpg"
                                                                            alt="Image Description"/>
                                                                        <img
                                                                            class="inline-block h-6 w-6 rounded-full ring-2 ring-white dark:ring-gray-700"
                                                                            src="images/users/avatar-2.jpg"
                                                                            alt="Image Description"/>
                                                                        <img
                                                                            class="inline-block h-6 w-6 rounded-full ring-2 ring-white dark:ring-gray-700"
                                                                            src="images/users/avatar-3.jpg"
                                                                            alt="Image Description"/>
                                                                        <img
                                                                            class="inline-block h-6 w-6 rounded-full ring-2 ring-white dark:ring-gray-700"
                                                                            src="images/users/avatar-4.jpg"
                                                                            alt="Image Description"/>
                                                                    </div>
                                                                </td>
                                                                <td class="p-3.5">
                                                                    <div>
                                                                        <button data-fc-type="dropdown"
                                                                                data-fc-placement="bottom-end"
                                                                                class="inline-flex text-slate-700 hover:bg-slate-100 dark:hover:bg-gray-700 dark:text-gray-300 rounded-full p-2">
                                                                            <i data-feather="more-vertical"
                                                                               class="w-4 h-4"></i>
                                                                        </button>

                                                                        <div
                                                                            class="fc-dropdown hidden fc-dropdown-open:opacity-100 opacity-0 w-40 z-50 mt-2 transition-all duration-300 bg-white dark:bg-gray-800 shadow-lg border border-gray-200 dark:border-gray-700 rounded-lg p-2">
                                                                            <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                                               href="apps-file-manager.html#">
                                                                                <i data-feather="edit-3"
                                                                                   class="w-4 h-4 me-3"></i>
                                                                                Edit
                                                                            </a>
                                                                            <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                                               href="apps-file-manager.html#">
                                                                                <i data-feather="link"
                                                                                   class="w-4 h-4 me-3"></i>
                                                                                Copy Link
                                                                            </a>
                                                                            <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                                               href="apps-file-manager.html#">
                                                                                <i data-feather="share-2"
                                                                                   class="w-4 h-4 me-3"></i>
                                                                                Share
                                                                            </a>
                                                                            <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                                               href="apps-file-manager.html#">
                                                                                <i data-feather="download"
                                                                                   class="w-4 h-4 me-3"></i>
                                                                                Download
                                                                            </a>
                                                                        </div>
                                                                    </div>
                                                                </td>
                                                            </tr>

                                                            <tr>
                                                                <td class="p-3.5 text-sm text-gray-700 dark:text-gray-400">
                                                                    <a href="javascript: void(0);"
                                                                       class="font-medium">Hyper-sketch-design.zip</a>
                                                                </td>
                                                                <td class="p-3.5 text-sm text-gray-700 dark:text-gray-400">
                                                                    <p>Feb 13, 2020</p>
                                                                    <span class="text-xs">by Coderthemes</span>
                                                                </td>
                                                                <td class="p-3.5 text-sm text-gray-700 dark:text-gray-400">521
                                                                    MB
                                                                </td>
                                                                <td class="p-3.5 text-sm text-gray-700 dark:text-gray-400">
                                                                    Coder Themes
                                                                </td>
                                                                <td class="p-3.5">
                                                                    <div class="flex -space-x-1.5">
                                                                        <img
                                                                            class="inline-block h-6 w-6 rounded-full ring-2 ring-white dark:ring-gray-700"
                                                                            src="images/users/avatar-4.jpg"
                                                                            alt="Image Description"/>
                                                                        <img
                                                                            class="inline-block h-6 w-6 rounded-full ring-2 ring-white dark:ring-gray-700"
                                                                            src="images/users/avatar-8.jpg"
                                                                            alt="Image Description"/>
                                                                        <img
                                                                            class="inline-block h-6 w-6 rounded-full ring-2 ring-white dark:ring-gray-700"
                                                                            src="images/users/avatar-6.jpg"
                                                                            alt="Image Description"/>
                                                                    </div>
                                                                </td>
                                                                <td class="p-3.5">
                                                                    <div>
                                                                        <button data-fc-type="dropdown"
                                                                                data-fc-placement="bottom-end"
                                                                                class="inline-flex text-slate-700 hover:bg-slate-100 dark:hover:bg-gray-700 dark:text-gray-300 rounded-full p-2">
                                                                            <i data-feather="more-vertical"
                                                                               class="w-4 h-4"></i>
                                                                        </button>

                                                                        <div
                                                                            class="fc-dropdown hidden fc-dropdown-open:opacity-100 opacity-0 w-40 z-50 mt-2 transition-all duration-300 bg-white dark:bg-gray-800 shadow-lg border border-gray-200 dark:border-gray-700 rounded-lg p-2">
                                                                            <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                                               href="apps-file-manager.html#">
                                                                                <i data-feather="edit-3"
                                                                                   class="w-4 h-4 me-3"></i>
                                                                                Edit
                                                                            </a>
                                                                            <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                                               href="apps-file-manager.html#">
                                                                                <i data-feather="link"
                                                                                   class="w-4 h-4 me-3"></i>
                                                                                Copy Link
                                                                            </a>
                                                                            <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                                               href="apps-file-manager.html#">
                                                                                <i data-feather="share-2"
                                                                                   class="w-4 h-4 me-3"></i>
                                                                                Share
                                                                            </a>
                                                                            <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                                               href="apps-file-manager.html#">
                                                                                <i data-feather="download"
                                                                                   class="w-4 h-4 me-3"></i>
                                                                                Download
                                                                            </a>
                                                                        </div>
                                                                    </div>
                                                                </td>
                                                            </tr>
                                                            <tr>
                                                                <td class="p-3.5 text-sm text-gray-700 dark:text-gray-400">
                                                                    <a href="javascript: void(0);"
                                                                       class="font-medium">Annualreport.pdf</a>
                                                                </td>
                                                                <td class="p-3.5 text-sm text-gray-700 dark:text-gray-400">
                                                                    <p>Dec 18, 2019</p>
                                                                    <span class="text-xs">by Alejandro</span>
                                                                </td>
                                                                <td class="p-3.5 text-sm text-gray-700 dark:text-gray-400">7.2
                                                                    MB
                                                                </td>
                                                                <td class="p-3.5 text-sm text-gray-700 dark:text-gray-400">
                                                                    Gary Coley
                                                                </td>
                                                                <td class="p-3.5">
                                                                    <div class="flex -space-x-1.5">
                                                                        <img
                                                                            class="inline-block h-6 w-6 rounded-full ring-2 ring-white dark:ring-gray-700"
                                                                            src="images/users/avatar-5.jpg"
                                                                            alt="Image Description"/>
                                                                        <img
                                                                            class="inline-block h-6 w-6 rounded-full ring-2 ring-white dark:ring-gray-700"
                                                                            src="images/users/avatar-7.jpg"
                                                                            alt="Image Description"/>
                                                                        <img
                                                                            class="inline-block h-6 w-6 rounded-full ring-2 ring-white dark:ring-gray-700"
                                                                            src="images/users/avatar-4.jpg"
                                                                            alt="Image Description"/>
                                                                    </div>
                                                                </td>
                                                                <td class="p-3.5">
                                                                    <div>
                                                                        <button data-fc-type="dropdown"
                                                                                data-fc-placement="bottom-end"
                                                                                class="inline-flex text-slate-700 hover:bg-slate-100 dark:hover:bg-gray-700 dark:text-gray-300 rounded-full p-2">
                                                                            <i data-feather="more-vertical"
                                                                               class="w-4 h-4"></i>
                                                                        </button>

                                                                        <div
                                                                            class="fc-dropdown hidden fc-dropdown-open:opacity-100 opacity-0 w-40 z-50 mt-2 transition-all duration-300 bg-white dark:bg-gray-800 shadow-lg border border-gray-200 dark:border-gray-700 rounded-lg p-2">
                                                                            <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                                               href="apps-file-manager.html#">
                                                                                <i data-feather="edit-3"
                                                                                   class="w-4 h-4 me-3"></i>
                                                                                Edit
                                                                            </a>
                                                                            <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                                               href="apps-file-manager.html#">
                                                                                <i data-feather="link"
                                                                                   class="w-4 h-4 me-3"></i>
                                                                                Copy Link
                                                                            </a>
                                                                            <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                                               href="apps-file-manager.html#">
                                                                                <i data-feather="share-2"
                                                                                   class="w-4 h-4 me-3"></i>
                                                                                Share
                                                                            </a>
                                                                            <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                                               href="apps-file-manager.html#">
                                                                                <i data-feather="download"
                                                                                   class="w-4 h-4 me-3"></i>
                                                                                Download
                                                                            </a>
                                                                        </div>
                                                                    </div>
                                                                </td>
                                                            </tr>
                                                            <tr>
                                                                <td class="p-3.5 text-sm text-gray-700 dark:text-gray-400">
                                                                    <a href="javascript: void(0);"
                                                                       class="font-medium">Wireframes</a>
                                                                </td>
                                                                <td class="p-3.5 text-sm text-gray-700 dark:text-gray-400">
                                                                    <p>Nov 25, 2019</p>
                                                                    <span class="text-xs">by Dunkle</span>
                                                                </td>
                                                                <td class="p-3.5 text-sm text-gray-700 dark:text-gray-400">54.2
                                                                    MB
                                                                </td>
                                                                <td class="p-3.5 text-sm text-gray-700 dark:text-gray-400">
                                                                    Jasper Rigg
                                                                </td>
                                                                <td class="p-3.5">
                                                                    <div class="flex -space-x-1.5">
                                                                        <img
                                                                            class="inline-block h-6 w-6 rounded-full ring-2 ring-white dark:ring-gray-700"
                                                                            src="images/users/avatar-6.jpg"
                                                                            alt="Image Description"/>
                                                                        <img
                                                                            class="inline-block h-6 w-6 rounded-full ring-2 ring-white dark:ring-gray-700"
                                                                            src="images/users/avatar-4.jpg"
                                                                            alt="Image Description"/>
                                                                        <img
                                                                            class="inline-block h-6 w-6 rounded-full ring-2 ring-white dark:ring-gray-700"
                                                                            src="images/users/avatar-7.jpg"
                                                                            alt="Image Description"/>
                                                                        <img
                                                                            class="inline-block h-6 w-6 rounded-full ring-2 ring-white dark:ring-gray-700"
                                                                            src="images/users/avatar-5.jpg"
                                                                            alt="Image Description"/>
                                                                    </div>
                                                                </td>
                                                                <td class="p-3.5">
                                                                    <div>
                                                                        <button data-fc-type="dropdown"
                                                                                data-fc-placement="bottom-end"
                                                                                class="inline-flex text-slate-700 hover:bg-slate-100 dark:hover:bg-gray-700 dark:text-gray-300 rounded-full p-2">
                                                                            <i data-feather="more-vertical"
                                                                               class="w-4 h-4"></i>
                                                                        </button>

                                                                        <div
                                                                            class="fc-dropdown hidden fc-dropdown-open:opacity-100 opacity-0 w-40 z-50 mt-2 transition-all duration-300 bg-white dark:bg-gray-800 shadow-lg border border-gray-200 dark:border-gray-700 rounded-lg p-2">
                                                                            <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                                               href="apps-file-manager.html#">
                                                                                <i data-feather="edit-3"
                                                                                   class="w-4 h-4 me-3"></i>
                                                                                Edit
                                                                            </a>
                                                                            <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                                               href="apps-file-manager.html#">
                                                                                <i data-feather="link"
                                                                                   class="w-4 h-4 me-3"></i>
                                                                                Copy Link
                                                                            </a>
                                                                            <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                                               href="apps-file-manager.html#">
                                                                                <i data-feather="share-2"
                                                                                   class="w-4 h-4 me-3"></i>
                                                                                Share
                                                                            </a>
                                                                            <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                                               href="apps-file-manager.html#">
                                                                                <i data-feather="download"
                                                                                   class="w-4 h-4 me-3"></i>
                                                                                Download
                                                                            </a>
                                                                        </div>
                                                                    </div>
                                                                </td>
                                                            </tr>
                                                            <tr>
                                                                <td class="p-3.5 text-sm text-gray-700 dark:text-gray-400">
                                                                    <a href="javascript: void(0);"
                                                                       class="font-medium">Documentation.docs</a>
                                                                </td>
                                                                <td class="p-3.5 text-sm text-gray-700 dark:text-gray-400">
                                                                    <p>Feb 9, 2020</p>
                                                                    <span class="text-xs">by Justin</span>
                                                                </td>
                                                                <td class="p-3.5 text-sm text-gray-700 dark:text-gray-400">8.3
                                                                    MB
                                                                </td>
                                                                <td class="p-3.5 text-sm text-gray-700 dark:text-gray-400">
                                                                    Cooper Sharwood
                                                                </td>
                                                                <td class="p-3.5">
                                                                    <div class="flex -space-x-1.5">
                                                                        <img
                                                                            class="inline-block h-6 w-6 rounded-full ring-2 ring-white dark:ring-gray-700"
                                                                            src="images/users/avatar-5.jpg"
                                                                            alt="Image Description"/>
                                                                        <img
                                                                            class="inline-block h-6 w-6 rounded-full ring-2 ring-white dark:ring-gray-700"
                                                                            src="images/users/avatar-8.jpg"
                                                                            alt="Image Description"/>
                                                                    </div>
                                                                </td>
                                                                <td class="p-3.5">
                                                                    <div>
                                                                        <button data-fc-type="dropdown"
                                                                                data-fc-placement="bottom-end"
                                                                                class="inline-flex text-slate-700 hover:bg-slate-100 dark:hover:bg-gray-700 dark:text-gray-300 rounded-full p-2">
                                                                            <i data-feather="more-vertical"
                                                                               class="w-4 h-4"></i>
                                                                        </button>

                                                                        <div
                                                                            class="fc-dropdown hidden fc-dropdown-open:opacity-100 opacity-0 w-40 z-50 mt-2 transition-all duration-300 bg-white dark:bg-gray-800 shadow-lg border border-gray-200 dark:border-gray-700 rounded-lg p-2">
                                                                            <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                                               href="apps-file-manager.html#">
                                                                                <i data-feather="edit-3"
                                                                                   class="w-4 h-4 me-3"></i>
                                                                                Edit
                                                                            </a>
                                                                            <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                                               href="apps-file-manager.html#">
                                                                                <i data-feather="link"
                                                                                   class="w-4 h-4 me-3"></i>
                                                                                Copy Link
                                                                            </a>
                                                                            <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                                               href="apps-file-manager.html#">
                                                                                <i data-feather="share-2"
                                                                                   class="w-4 h-4 me-3"></i>
                                                                                Share
                                                                            </a>
                                                                            <a class="flex items-center py-2 px-4 text-sm rounded text-gray-500  hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
                                                                               href="apps-file-manager.html#">
                                                                                <i data-feather="download"
                                                                                   class="w-4 h-4 me-3"></i>
                                                                                Download
                                                                            </a>
                                                                        </div>
                                                                    </div>
                                                                </td>
                                                            </tr>
                                                            </tbody>
                                                        </table>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </main>

                {/*   <!-- Footer Start  --> */}
                <footer class="footer h-16 flex items-center px-6 bg-white shadow dark:bg-gray-800">
                    <div class="flex justify-center w-full gap-4">
                        <div>
                            {/*<script>document.write(new Date().getFullYear())</script> © Konrix - <a href="https://coderthemes.com/" target="_blank">Coderthemes</a>*/}
                        </div>
                    </div>
                </footer>
                {/*   <!-- Footer End  --> */}

            </div>

            {/*    <!-- ==============================================================  --> */}
            {/*   <!-- End Page content  --> */}
            {/*   <!-- ==============================================================  --> */}

        </div>
    );
};

export default App;
