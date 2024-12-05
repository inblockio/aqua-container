import { LuImport, LuScan, LuUpload } from "react-icons/lu";
import { Button } from "./ui/button";
import axios from "axios";
import { useStore } from "zustand";
import appStore from "../store";
import { useEffect, useState } from "react";
import { ApiFileInfo } from "../models/FileInfo";
import { toaster } from "./ui/toaster";
import { ENDPOINTS } from "../utils/constants";
import { readJsonFile } from "../utils/functions";
import ChainDetails from "./ui/navigation/CustomDrawer";


interface IDropzoneAction {
    file: File
    fileIndex: number
    uploadedIndexes: number[]
    updateUploadedIndex: (fileIndex: number) => void
}

export const UploadFile = ({ file, uploadedIndexes, fileIndex, updateUploadedIndex }: IDropzoneAction) => {

    const [uploading, setUploading] = useState(false)
    const [uploaded, setUploaded] = useState(false)

    const { metamaskAddress, setFiles, files } = useStore(appStore)

    const uploadFile = async () => {
        if (!file) {
            toaster.create({
                description: "No file selected!",
                type: "info"
            })
            return;
        }

        const formData = new FormData();
        formData.append('file', file);
        formData.append('account', `${metamaskAddress}`);

        setUploading(true)
        try {
            const response = await axios.post(ENDPOINTS.UPLOAD_FILE, formData, {
                headers: {
                    'Content-Type': 'multipart/form-data',
                    "metamask_address": metamaskAddress
                },
            });

            const res = response.data

            // let logs: Array<string> = res.logs
            // logs.forEach((item) => {
            //     console.log("**>" + item + "\n.")
            // })


            // Assuming the API returns an array of FileInfo objects
            const file: ApiFileInfo = {
                id: res.file.id,
                name: res.file.name,
                extension: res.file.extension,
                page_data: res.file.page_data,
                mode: res.file.mode,
                owner: res.file.owner
            };

            setFiles([...files, file])
            setUploaded(true)
            setUploading(false)
            toaster.create({
                description: "File uploaded successfuly",
                type: "success"
            })
            updateUploadedIndex(fileIndex)
            return;
        } catch (error) {
            setUploading(false)
            toaster.create({
                description: `Failed to upload file: ${error}`,
                type: "error"
            })
        }
    };

    return (
        <Button size={'xs'} colorPalette={'blackAlpha'} variant={'subtle'} w={'80px'} onClick={uploadFile} disabled={uploadedIndexes.includes(fileIndex) || uploaded} loading={uploading}>
            <LuUpload />
            Upload
        </Button>
    )
}


export const VerifyFile = ({ file }: IDropzoneAction) => {

    const [verifying, setVerifying] = useState(false)
    const [hashChainForVerification, setHashChain] = useState<ApiFileInfo>()
    // const [uploaded, setUploaded] = useState(false)

    // const { metamaskAddress, setFiles, files } = useStore(appStore)

    const handleVerifyAquaJsonFile = () => {
        setVerifying(true)
        readJsonFile(file)
            .then((jsonData) => {
                const hashChain: ApiFileInfo = {
                    id: 0,
                    name: '',
                    extension: '',
                    page_data: JSON.stringify(jsonData),
                    mode: '',
                    owner: ''
                }
                setHashChain(hashChain)
                // const hashChainString = JSON.stringify(hashChain)
                // console.log("JSON data:", hashChain);
                // setAppState("selectedFileFromApi", hashChain);
                // navigate("/details");
                // Handle the JSON data here
            })
            .catch(() => {
                // Handle the error here
            });
        setVerifying(false)
    };

    useEffect(() => {
        handleVerifyAquaJsonFile()
    }, [])

    return (
        <>
            {
                hashChainForVerification ? (
                    <ChainDetails fileInfo={hashChainForVerification} />
                ) : (
                    <Button size={'xs'} colorPalette={'blackAlpha'} variant={'subtle'} w={'80px'} loading={verifying} disabled>
                        <LuScan />
                        LOading Chain
                    </Button>
                )
            }
        </>
    )
}


export const ImportAquaChain = ({ file, uploadedIndexes, fileIndex, updateUploadedIndex }: IDropzoneAction) => {

    const [uploading, setUploading] = useState(false)
    const [uploaded, setUploaded] = useState(false)

    const { metamaskAddress, setFiles, files, user_profile } = useStore(appStore)

    const importAquaChain = async () => {

        if (!file) {
            toaster.create({
                description: "No file selected!",
                type: "error"
            })
            return;
        }

        const formData = new FormData();
        formData.append('file', file);
        formData.append('account', "example");
        setUploading(true)
        try {
            const response = await axios.post(ENDPOINTS.IMPORT_AQUA_CHAIN, formData, {
                headers: {
                    'Content-Type': 'multipart/form-data',
                    "metamask_address": metamaskAddress
                },
            });

            const res = response.data

            // let logs: Array<string> = res.logs
            // logs.forEach((item) => {
            //     console.log("**>" + item + "\n.")
            // })

            // console.log("Upload res: ", res)

            // Assuming the API returns an array of FileInfo objects
            const file: ApiFileInfo = {
                id: res.file.id,
                name: res.file.name,
                extension: res.file.extension,
                page_data: res.file.page_data,
                mode: user_profile.fileMode ?? "",
                owner: metamaskAddress ?? "",
            };
            setFiles([...files, file])
            // setUploadedFilesIndexes(value => [...value, fileIndex])
            toaster.create({
                description: "Aqua Chain imported successfully",
                type: "success"
            })
            setUploading(false)
            setUploaded(true)
            updateUploadedIndex(fileIndex)
            return;
        } catch (error) {
            setUploading(false)
            toaster.create({
                description: `Failed to import chain: ${error}`,
                type: "error"
            })
        }
    };

    return (
        <Button size={'xs'} colorPalette={'blackAlpha'} variant={'subtle'} w={'80px'} onClick={importAquaChain} disabled={uploadedIndexes.includes(fileIndex) || uploaded} loading={uploading}>
            <LuImport />
            Import
        </Button>
    )
}