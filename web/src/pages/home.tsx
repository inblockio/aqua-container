import type { Component } from 'solid-js';
import { createEffect, createSignal, For } from "solid-js";
import axios from "axios";
import { ethers } from "ethers";
import { FileInfo } from "../models/FileInfo";
import { getTimestampSafe, PageData, Revision } from "../models/PageData";
import {
    capitalizeFirstLetter,
    debugPageDataStructure, fileType,
    filterFilesByType,
    humanReadableFileSize,
    sumFileContentSizes, timeToHumanFriendly
} from "../util";
import { UiFileTypes } from "../models/UiFileTypes";
import { appState, setAppState } from "../store/store";
import app from "../App";
import { useNavigate } from "@solidjs/router";
import SignFile from '../components/SignFile';
import WitnessFile from '../components/WitnessFile';
import MainLayout from '../layout/MainLayout';
import { API_BASE_ENDPOINT } from '../config/constants';
import { fetchFiles } from '../network/api';

const HomePage: Component = () => {


    const navigate = useNavigate();
    const [selectedFileForUpload, setSelectedFileForUpload] = createSignal<File | null>(null);
    // const [fileFromApi, setFilesFromApi] = createSignal<Array<FileInfo>>([]);
    const [allFilesSize, setAllFileSize] = createSignal<number>(0);
    const [pageDataInfo, setPageDataInfo] = createSignal("");
    const [fileTypeForUpload, setFileTypeForUpload] = createSignal("");
    const [error, setError] = createSignal<string>('');
    const [success, setSuccess] = createSignal<string>('');
    const maxFileSize = 20 * 1024 * 1024; // 20 MB in bytes

    let fileInput: HTMLInputElement;

    // createEffect(async () => {

    //     let files = await fetchFiles("");
    //     setAppState('filesFromApi', files);

    //     let size = 0;
    //     for (const element of files) {
    //         const pageData: PageData = JSON.parse(element.page_data);
    //         // Debug the structure
    //         debugPageDataStructure(pageData);

    //         let currentSize = sumFileContentSizes(pageData)
    //         size += currentSize
    //     }

    //     setAllFileSize(size)
    //     let hSize = humanReadableFileSize(size)
    //     let info = `${files.length} files (${hSize})`
    //     setPageDataInfo(info)
    // })


    createEffect(async () => {

        
        let files = appState.filesFromApi;
        let size = 0;
        for (const element of files) {
            const pageData: PageData = JSON.parse(element.page_data);
            // Debug the structure
            debugPageDataStructure(pageData);

            let currentSize = sumFileContentSizes(pageData)
            size += currentSize
        }

        setAllFileSize(size)
        let hSize = humanReadableFileSize(size)
        let info = `${files.length} files (${hSize})`
        setPageDataInfo(info)

    }, [appState.filesFromApi])




    const uploadAquaJsonFile = async () => {
        const file = selectedFileForUpload();
        console.log("Uploading")
        if (!file) {
            setError('No file selected');
            return;
        }

        const formData = new FormData();
        formData.append('file', file);
        formData.append('account', "example");

        try {
            const response = await axios.post('http://127.0.0.1:3600/explorer_verify_hash', formData, {
                headers: {
                    'Content-Type': 'multipart/form-data',
                },
            });
            console.log('File uploaded successfully:', JSON.stringify(response.data));


            console.log("Code " + response.status)

            if (response.status == 200) {
                setSuccess("Verification success")
            } else {
                setError("Verification failed")
            }

            setFileTypeForUpload("");
            return;
        } catch (error: any) {
            if (error.response && error.response.status === 404) {
                console.error('Custom 404 message: The requested resource was not found', error.response);
                setError('404 : Failed to upload file ' + error.response);

            } else if (error.response && error.response.status === 400) {
                console.error('Custom 400', error.response);
                setError('400 Failed to upload file ' + error.response.data);

            } else {
                console.error('An error occurred:', error.message);
                console.error('Error uploading file:', error);
                setError(error.message + " =--- " + error.message);
            }

        }

        // axios.post('http://127.0.0.1:3600/explorer_verify_hash', formData, {
        //     headers: {
        //         'Content-Type': 'multipart/form-data',
        //     },
        // }).then((res: any) => {

        // }).catch(error => {
        //     console.log("Error", error)
        // })
    }

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

        // Get the file name
        const fileName = file.name;
        console.log('Selected file:', fileName);

        let exists = appState.filesFromApi.find((e) => e.name == fileName);

        if (exists != undefined && fileTypeForUpload() !== "json") {

            setSelectedFileForUpload(null);
            setError('Rename the file, a file with name ' + fileName + ' already exists');
            // Reset the input
            input.value = '';
            return;

        } else {
            setSelectedFileForUpload(file);
            setError('');

            console.log("Type: ", fileTypeForUpload())
            if (fileTypeForUpload() === "json") {
                console.log("Uploading json")
                uploadAquaJsonFile()
            } else {
                //upload the file
                console.log("Uploading file")
                uploadFile()
            }
        }
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
                    "metamask_address": appState.metaMaskAddress
                },
            });
            console.log('File uploaded successfully:', JSON.stringify(response.data));


            setSelectedFileForUpload(null);

            const res = response.data

            let logs: Array<string> = res.logs
            logs.forEach((item) => {
                console.log("**>" + item + "\n.")
            })


            // Assuming the API returns an array of FileInfo objects
            const file: FileInfo = {
                id: res.file.id,
                name: res.file.name,
                extension: res.file.extension,
                page_data: res.file.page_data
            };

            setAppState("filesFromApi", [...appState.filesFromApi, file])

            setFileTypeForUpload("")
            return;
        } catch (error) {
            console.error('Error uploading file:', error);
            setError('Failed to upload file');
        }
    };



    const downloadAquaJson = (fileInfo: FileInfo) => {
        try {
            // Parse the page_data string to a PageData object
            const pageData: PageData = JSON.parse(fileInfo.page_data);

            for (const page of pageData.pages) {
                for (const revisionKey in page.revisions) {
                    const revision = page.revisions[revisionKey];

                    // Check if the revision has a witness and update witness_event_transaction_hash
                    if (revision.witness && revision.witness.witness_event_transaction_hash) {
                        revision.witness.witness_event_transaction_hash = `0x${revision.witness.witness_event_transaction_hash}`;
                    }
                }
            }

            // Convert the PageData object to a formatted JSON string
            const jsonString = JSON.stringify(pageData, null, 2);

            // Create a Blob from the JSON string
            const blob = new Blob([jsonString], { type: 'application/json' });

            // Create a URL for the Blob
            const url = URL.createObjectURL(blob);

            // Create a temporary anchor element and trigger the download
            const a = document.createElement('a');
            a.href = url;
            a.download = `${fileInfo.name}-page-data.json`;
            document.body.appendChild(a);
            a.click();

            // Clean up
            document.body.removeChild(a);
            URL.revokeObjectURL(url);
        } catch (error) {
            console.error('Error downloading JSON:', error);
        }

    }

    const showFileTypesCards = () => {

        let fileTypes = ["image", "document", "music", "video"];

        let filesUiState: Array<UiFileTypes> = [];

        if (appState.filesFromApi.length > 0) {
            for (const element of fileTypes) {

                let fileItemData = filterFilesByType(appState.filesFromApi, element)

                let size = 0;
                for (const element of fileItemData) {
                    const pageData: PageData = JSON.parse(element.page_data);
                    // Debug the structure
                    debugPageDataStructure(pageData);

                    let currentSize = sumFileContentSizes(pageData)
                    size += currentSize
                }

                // console.log("element " + element + " length " + fileItemData.length + "  file " + element + " size  " + size);
                let percentage = size / allFilesSize() * 100
                let usingText = `Using ${percentage.toFixed(2)}% of storage`
                let hSize = humanReadableFileSize(size)

                let item: UiFileTypes = {
                    name: element,
                    usingText: usingText,
                    size: hSize,
                    totalFiles: `${fileItemData.length} Files`
                }

                filesUiState.push(item)

            }

        } else {
            for (const element of fileTypes) {
                let item: UiFileTypes = {
                    name: element,
                    usingText: `Using 0% of storage`,
                    size: "0",
                    totalFiles: `0 Files`
                }

                filesUiState.push(item)

            }
        }

        return <For each={filesUiState}>
            {(item, index) => (
                <>
                    {fileTypeCardItem(item)}
                </>
            )}
        </For>

    }

    const fileTypeCardItem = (fileData: UiFileTypes) => {
        return <div class="card">
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
                                <p class="font-semibold text-base">{capitalizeFirstLetter(fileData.name, true)}</p>
                                <p class="text-xs">{fileData.usingText}</p>
                            </div>
                        </div>
                        <div class="flex items-center absolute top-0 end-0">
                            <button data-fc-type="dropdown" data-fc-placement="bottom-end"
                                class="inline-flex text-slate-700 hover:bg-slate-100 dark:hover:bg-gray-700 dark:text-gray-300 rounded-full p-2">
                                <i data-feather="more-vertical" class="w-4 h-4"></i>
                            </button>


                        </div>
                    </div>
                    <div class="flex items-center gap-2">
                        <p class="text-sm">{fileData.size}</p>
                        <span class="p-0.5 bg-gray-600 rounded-full"></span>
                        <p class="text-sm">{fileData.totalFiles}</p>
                    </div>
                </div>
            </div>
            {/*   <!-- end card body  --> */}
        </div>
    }

    const fileListDisplay = (file: FileInfo) => {

        const pageData: PageData = JSON.parse(file.page_data);
        // Debug the structure
        debugPageDataStructure(pageData);

        let currentSize = sumFileContentSizes(pageData)
        let hSize = humanReadableFileSize(currentSize)

        const fileTypeInfo = fileType(file);

        const timeStamp = getTimestampSafe(pageData)
        let dateDisplay = "--"
        if (timeStamp != undefined) {
            dateDisplay = timeToHumanFriendly(timeStamp)
        }

        const getLastRevisionVerificationHash = () => {
            const revisionHashes = Object.keys(pageData.pages[0].revisions)
            return pageData.pages[0].revisions[revisionHashes[revisionHashes.length - 1]].metadata.verification_hash
        }

        const getPreviousRevisionVerificationHash = () => {
            const revisionHashes = Object.keys(pageData.pages[0].revisions)
            return pageData.pages[0].revisions[revisionHashes[revisionHashes.length - 1]].metadata.verification_hash
        }

        return <tr >
            <td class="p-3.5 text-sm text-gray-700 dark:text-gray-400">
                <a href="javascript: void(0);" class="font-medium">
                    {file.name}</a>
            </td>
            <td class="p-3.5 text-sm text-gray-700 dark:text-gray-400">
                {fileTypeInfo}

            </td>
            <td class="p-3.5 text-sm text-gray-700 dark:text-gray-400">
                <p> {dateDisplay}</p>
                {/*<span class="text-xs">by Andrew</span>*/}
            </td>
            <td class="p-3.5 text-sm text-gray-700 dark:text-gray-400">
                {hSize}
            </td>

            {/* <td class="p-3.5">
                <div></div>
            </td> */}
            <td class="">
                <div>

                    <div onClick={(e) => {
                        downloadAquaJson(file)
                    }} class="rounded bg-success/25 px-2 " style="display: inline-flex; align-items: center;">


                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-download m-2" viewBox="0 0 16 16">
                            <path d="M.5 9.9a.5.5 0 0 1 .5.5v2.5a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-2.5a.5.5 0 0 1 1 0v2.5a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2v-2.5a.5.5 0 0 1 .5-.5" />
                            <path d="M7.646 11.854a.5.5 0 0 0 .708 0l3-3a.5.5 0 0 0-.708-.708L8.5 10.293V1.5a.5.5 0 0 0-1 0v8.793L5.354 8.146a.5.5 0 1 0-.708.708z" />
                        </svg>  Download Aqua-File

                    </div>

                    <div onClick={(e) => {
                        setAppState("selectedFileFromApi", file);
                        navigate("/details");
                    }} class="rounded bg-warning/25 px-2 mx-4 " style="display: inline-flex; align-items: center;">

                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-eye-fill m-2" viewBox="0 0 16 16">
                            <path d="M10.5 8a2.5 2.5 0 1 1-5 0 2.5 2.5 0 0 1 5 0" />
                            <path d="M0 8s3-5.5 8-5.5S16 8 16 8s-3 5.5-8 5.5S0 8 0 8m8 3.5a3.5 3.5 0 1 0 0-7 3.5 3.5 0 0 0 0 7" />
                        </svg>
                        See Details
                    </div>

                    <SignFile pageVerificationHash={getLastRevisionVerificationHash()} filename={file.name} />
                    <WitnessFile previousVerificationHash={getPreviousRevisionVerificationHash()} filename={file.name} />

                    <div onClick={async (e) => {
                        const formData = new URLSearchParams();
                        formData.append('filename', file.name);

                        const response = await axios.post(`${API_BASE_ENDPOINT}/explorer_delete_file`, formData, {
                            headers: {
                                'Content-Type': 'application/x-www-form-urlencoded'
                            }
                        });
                        console.log("Files Delete  response code " + response.status + "\n ");

                        if (response.status === 200) {
                            let files = appState.filesFromApi
                            let filesNew: Array<FileInfo> = [];
                            for (let index = 0; index < files.length; index++) {
                                const element = files[index];
                                if (element.name != file.name) {
                                    filesNew.push(element)
                                }
                            }
                            setAppState("filesFromApi", filesNew);
                        }
                    }} class="rounded bg-danger/25 px-2 mx-4 " style="display: inline-flex; align-items: center;">

                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-trash m-2" viewBox="0 0 16 16">
                            <path d="M5.5 5.5A.5.5 0 0 1 6 6v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5m2.5 0a.5.5 0 0 1 .5.5v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5m3 .5a.5.5 0 0 0-1 0v6a.5.5 0 0 0 1 0z" />
                            <path d="M14.5 3a1 1 0 0 1-1 1H13v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4h-.5a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1H6a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1h3.5a1 1 0 0 1 1 1zM4.118 4 4 4.059V13a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1V4.059L11.882 4zM2.5 3h11V2h-11z" />
                        </svg>
                        Delete
                    </div>
                </div>
            </td>
        </tr>
    }
    return (
        <>
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
                                            if(appState.metaMaskAddress == null){
                                                setError("Please sign in with meta mask wallet address to upload a file");
                                                return;
                                            }
                                            setFileTypeForUpload("file")
                                            handleSelectFileForUploadClick()
                                        }} data-fc-type="dropdown" data-fc-placement="bottom"
                                            type="button"
                                            class="btn inline-flex justify-center items-center bg-primary text-white w-full mb-3">
                                            <i class="mgc_add_line text-lg me-2"></i> Upload File
                                        </a>
                                        <br />

                                        <a href="javascript:void(0)" onClick={(e) => {
                                            setFileTypeForUpload("json")
                                            handleSelectFileForUploadClick();
                                        }} data-fc-type="dropdown" data-fc-placement="bottom"
                                            type="button"
                                            class="btn inline-flex justify-center items-center bg-primary text-white w-full mt-4">
                                            <i class="mgc_add_line text-lg me-2"></i> Verify Aqua-File
                                        </a>
                                        {/* Hidden file input */}
                                        <input
                                            type="file"
                                            ref={el => fileInput = el} // Save reference to the input element
                                            style={{ display: "none" }} // Hide the input element
                                            onChange={handleFileSelect}
                                        />

                                        <a onClick={(e) => {
                                            e.preventDefault();
                                            console.log("Woopsiiiee ...........");
                                            navigate("/configuration")
                                        }} href="javascript:void(0);" class="flex items-center py-2 px-4 text-sm rounded text-gray-500 hover:bg-slate-100 hover:text-slate-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300 mt-5" id="headingOne">
                                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-gear my-2" viewBox="0 0 16 16">
                                                <path d="M8 4.754a3.246 3.246 0 1 0 0 6.492 3.246 3.246 0 0 0 0-6.492M5.754 8a2.246 2.246 0 1 1 4.492 0 2.246 2.246 0 0 1-4.492 0" />
                                                <path d="M9.796 1.343c-.527-1.79-3.065-1.79-3.592 0l-.094.319a.873.873 0 0 1-1.255.52l-.292-.16c-1.64-.892-3.433.902-2.54 2.541l.159.292a.873.873 0 0 1-.52 1.255l-.319.094c-1.79.527-1.79 3.065 0 3.592l.319.094a.873.873 0 0 1 .52 1.255l-.16.292c-.892 1.64.901 3.434 2.541 2.54l.292-.159a.873.873 0 0 1 1.255.52l.094.319c.527 1.79 3.065 1.79 3.592 0l.094-.319a.873.873 0 0 1 1.255-.52l.292.16c1.64.893 3.434-.902 2.54-2.541l-.159-.292a.873.873 0 0 1 .52-1.255l.319-.094c1.79-.527 1.79-3.065 0-3.592l-.319-.094a.873.873 0 0 1-.52-1.255l.16-.292c.893-1.64-.902-3.433-2.541-2.54l-.292.159a.873.873 0 0 1-1.255-.52zm-2.633.283c.246-.835 1.428-.835 1.674 0l.094.319a1.873 1.873 0 0 0 2.693 1.115l.291-.16c.764-.415 1.6.42 1.184 1.185l-.159.292a1.873 1.873 0 0 0 1.116 2.692l.318.094c.835.246.835 1.428 0 1.674l-.319.094a1.873 1.873 0 0 0-1.115 2.693l.16.291c.415.764-.42 1.6-1.185 1.184l-.291-.159a1.873 1.873 0 0 0-2.693 1.116l-.094.318c-.246.835-1.428.835-1.674 0l-.094-.319a1.873 1.873 0 0 0-2.692-1.115l-.292.16c-.764.415-1.6-.42-1.184-1.185l.159-.291A1.873 1.873 0 0 0 1.945 8.93l-.319-.094c-.835-.246-.835-1.428 0-1.674l.319-.094A1.873 1.873 0 0 0 3.06 4.377l-.16-.292c-.415-.764.42-1.6 1.185-1.184l.292.159a1.873 1.873 0 0 0 2.692-1.115z" />
                                            </svg>
                                            &emsp;
                                            <span>Config</span>
                                        </a>
                                    </div>


                                    <div class="mt-6">
                                        <h6 class="text-uppercase mt-3">Storage</h6>
                                        <div
                                            class="flex w-full h-1.5 bg-gray-200 rounded-full overflow-hidden dark:bg-gray-700 mt-4">
                                            <div class="flex flex-col justify-center overflow-hidden bg-primary"
                                                role="progressbar" style="width: 46%" aria-valuenow="46" aria-valuemin="0"
                                                aria-valuemax="100"></div>
                                        </div>
                                        <p class="text-gray-500 mt-4 text-xs">{pageDataInfo()}</p>
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
                                                        class="bg-red-500 text-sm text-white transition duration-300 bg-danger-50 border border-red-200 rounded-md p-4"
                                                        role="alert">
                                                        <div class="flex items-center gap-3">
                                                            <div class="flex-shrink-0">
                                                                <i class="mgc_-badge-check text-xl"></i>
                                                            </div>
                                                            <div class="flex-grow">
                                                                <div class="text-sm text-white-800 font-medium">
                                                                    {error()}

                                                                </div>
                                                            </div>
                                                            <button onClick={(e) => {
                                                                setError("");
                                                                setSelectedFileForUpload(null)
                                                            }} data-fc-dismiss="dismiss-alert" type="button"
                                                                id="dismiss-test"
                                                                class="ms-auto h-8 w-8 rounded-full bg-white-400 flex justify-center items-center">
                                                                {/*<i class="mgc_close_line text-xl"></i>*/}
                                                                <svg xmlns="http://www.w3.org/2000/svg" width="16"
                                                                    height="16" fill="currentColor" class="bi bi-x"
                                                                    viewBox="0 0 16 16">
                                                                    <path
                                                                        d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708" />
                                                                </svg>
                                                            </button>
                                                        </div>
                                                    </div>
                                                    <br />
                                                </>
                                        }

                                        {
                                            success().length == 0 ? <div></div> :
                                                <>


                                                    <div id="dismiss-alert"
                                                        class="bg-green-500 text-sm text-white transition duration-300 bg-teal-50 border border-teal-200 rounded-md p-4"
                                                        role="alert">
                                                        <div class="flex items-center gap-3">
                                                            <div class="flex-shrink-0">
                                                                <i class="mgc_-badge-check text-xl"></i>
                                                            </div>
                                                            <div class="flex-grow">
                                                                <div class="text-sm text-teal-800 font-medium">
                                                                    {success()}

                                                                </div>
                                                            </div>
                                                            <button onClick={(e) => {
                                                                setSuccess("");
                                                                setSelectedFileForUpload(null)
                                                            }} data-fc-dismiss="dismiss-alert" type="button"
                                                                id="dismiss-test"
                                                                class="ms-auto h-8 w-8 rounded-full bg-gray-200 flex justify-center items-center">
                                                                {/*<i class="mgc_close_line text-xl"></i>*/}
                                                                <svg xmlns="http://www.w3.org/2000/svg" width="16"
                                                                    height="16" fill="currentColor" class="bi bi-x"
                                                                    viewBox="0 0 16 16">
                                                                    <path
                                                                        d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708" />
                                                                </svg>
                                                            </button>
                                                        </div>
                                                    </div>
                                                    <br />
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
                                        </div>
                                    </div>

                                    {showFileTypesCards()}


                                    <div class="2xl:col-span-4 sm:col-span-2">
                                        <div class="card">
                                            <div class="card-header">
                                                <h4 class="text-lg font-semibold text-gray-800 dark:text-gray-300">Files</h4>
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
                                                                            class="p-3.5 text-sm text-start font-semibold min-w-[10rem]">
                                                                            Type
                                                                        </th>
                                                                        <th scope="col"
                                                                            class="p-3.5 text-sm text-start font-semibold min-w-[10rem]">Uploaded
                                                                            At
                                                                        </th>
                                                                        <th scope="col"
                                                                            class="p-3.5 text-sm text-start font-semibold min-w-[6rem]">File
                                                                            Size
                                                                        </th>
                                                                        {/* <th scope="col"
                                                                        class="p-3.5 text-sm text-start font-semibold min-w-[8rem]">Owner
                                                                    </th> */}
                                                                        {/*<th scope="col"*/}
                                                                        {/*    class="p-3.5 text-sm text-start font-semibold min-w-[6rem]">Members*/}
                                                                        {/*</th>*/}
                                                                        <th scope="col"
                                                                            class="p-3.5 text-sm text-start font-semibold">Action
                                                                        </th>
                                                                    </tr>
                                                                </thead>
                                                                <tbody
                                                                    class="divide-y divide-gray-200 dark:divide-gray-600">

                                                                    <For each={appState.filesFromApi}>
                                                                        {(item, index) =>
                                                                            <>
                                                                                {fileListDisplay(item)}
                                                                            </>
                                                                        }

                                                                    </For>


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
        </>
    );
};

export default HomePage;
