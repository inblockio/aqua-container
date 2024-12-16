import { LuImport, LuScan, LuUpload } from "react-icons/lu";
import { Button } from "./ui/button";
import axios from "axios";
import { useStore } from "zustand";
import appStore from "../store";
import { useEffect, useState } from "react";
import { ApiFileInfo } from "../models/FileInfo";
import { toaster } from "./ui/toaster";
import { readJsonFile } from "../utils/functions";
import { ChainDetailsBtn } from "./ui/navigation/CustomDrawer";
import { Container, Group, Text } from "@chakra-ui/react";
import { Alert } from "./ui/alert";
import { useNavigate } from "react-router-dom";


interface IDropzoneAction {
    file: File
    fileIndex: number
    uploadedIndexes: number[]
    updateUploadedIndex: (fileIndex: number) => void
}

export const UploadFile = ({ file, uploadedIndexes, fileIndex, updateUploadedIndex }: IDropzoneAction) => {

    const [uploading, setUploading] = useState(false)
    const [uploaded, setUploaded] = useState(false)

    const { metamaskAddress, setFiles, files, backend_url } = useStore(appStore)

    const uploadFile = async () => {

        const existingChainFile = files.find(_file => file.name === _file.name)

        if (existingChainFile) {
            toaster.create({
                description: "You already have the file. Delete before importing this",
                type: "info"
            })
            return
        }


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
            const url = `${backend_url}/explorer_file_upload`
            console.log("url ", url)
            const response = await axios.post(url, formData, {
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
    const [_isVerificationSuccessful, setIsVerificationSuccessful] = useState(false)
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
                    <ChainDetailsBtn fileInfo={hashChainForVerification} callBack={(res) => setIsVerificationSuccessful(res)} />
                ) : (
                    <Button size={'xs'} colorPalette={'blackAlpha'} variant={'subtle'} w={'80px'} loading={verifying} disabled>
                        <LuScan />
                        Loading Chain
                    </Button>
                )
            }
        </>
    )
}


export const ImportAquaChainFromFile = ({ file, uploadedIndexes, fileIndex, updateUploadedIndex }: IDropzoneAction) => {

    const [uploading, setUploading] = useState(false)
    const [uploaded, setUploaded] = useState(false)

    const { metamaskAddress, setFiles, files, user_profile, backend_url } = useStore(appStore)

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
            const url = `${backend_url}/explorer_aqua_file_upload`;
            console.log("importAquaChain url ", url)
            const response = await axios.post(url, formData, {
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

interface ImportChainFromChainProps { fileInfo: ApiFileInfo, isVerificationSuccessful: boolean }

export const ImportAquaChainFromChain = ({ fileInfo, isVerificationSuccessful }: ImportChainFromChainProps) => {

    const [uploading, setUploading] = useState(false)
    const [_uploaded, setUploaded] = useState(false)
    const [dbFiles, setDbFiles] = useState<ApiFileInfo[]>([])

    const { metamaskAddress, setFiles, files, user_profile, backend_url } = useStore(appStore)

    let navigate =  useNavigate();

    const importAquaChain = async () => {

        // const existingChainFile = dbFiles.find(file => file.name === fileInfo.name)
        const chainToImport = JSON.parse(fileInfo.page_data).pages[0]
        const existingChainFile = dbFiles.find(file => JSON.parse(file.page_data).pages[0].genesis_hash === chainToImport.genesis_hash)
        
        if (existingChainFile) {
            const fileToImportRevisions = Object.keys(chainToImport.revisions)
            const existingFileRevisions = Object.keys(JSON.parse(existingChainFile.page_data).pages[0].revisions)

            console.log(fileToImportRevisions, existingFileRevisions)

            toaster.create({
                description: `You already have the file called "${fileInfo.name}". Delete before importing this `,
                type: "error"
            })
            return
        }

        // Create a JSON file from the page_data object
        const fileData = fileInfo.page_data // JSON.stringify(fileInfo.page_data, null, 2); // Convert to JSON string
        const file = new File([fileData], fileInfo.name, {
            type: "application/json",
        });

        if (!file) {
            toaster.create({
                description: "No file selected!",
                type: "error"
            })
            return;
        }

        const formData = new FormData();
        formData.append('file', file);
        formData.append('account', `${metamaskAddress}`);

        setUploading(true)

        try {
            const url = `${backend_url}/explorer_aqua_file_upload`
            console.log("importAquaChain url ", url)
            const response = await axios.post(url, formData, {
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
            navigate("/")
            return;
        } catch (error) {
            setUploading(false)
            toaster.create({
                description: `Failed to import chain: ${error}`,
                type: "error"
            })
        }
    };

    useEffect(() => {
        setDbFiles(files)
    }, [files])

    return (
        <Container maxW={'xl'}>
            <Alert title="Import Aqua Chain" icon={<LuImport />}>
                <Group gap={"10"}>
                    <Text>
                        Do you want to import this Aqua Chain?
                    </Text>
                    <Button size={'lg'} colorPalette={'blue'} variant={'solid'} onClick={importAquaChain} disabled={!isVerificationSuccessful} loading={uploading}>
                        <LuImport />
                        Import
                    </Button>
                </Group>
            </Alert>
            {/* <Alert.Root colorPalette={'orange'}>
                Would you like to import the file?
                <Button size={'lg'} colorPalette={'blue'} variant={'subtle'} onClick={importAquaChain} disabled={!isVerificationSuccessful} loading={uploading}>
                    <LuImport />
                    Import
                </Button>
            </Alert.Root> */}
        </Container>
    )
}