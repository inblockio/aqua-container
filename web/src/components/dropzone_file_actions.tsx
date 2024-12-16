import { LuCheck, LuChevronRight, LuImport, LuPackage, LuScan, LuShip, LuUpload, LuX } from "react-icons/lu";
import { Button } from "./ui/button";
import axios from "axios";
import { useStore } from "zustand";
import appStore from "../store";
import { useEffect, useState } from "react";
import { ApiFileInfo } from "../models/FileInfo";
import { toaster } from "./ui/toaster";
import { formatCryptoAddress, readJsonFile } from "../utils/functions";
import { ChainDetailsBtn } from "./ui/navigation/CustomDrawer";
import { Container, DialogCloseTrigger, Group, List, Text } from "@chakra-ui/react";
import { Alert } from "./ui/alert";
import { useNavigate } from "react-router-dom";
import { analyzeAndMergeRevisions } from "../utils/aqua_funcs";
import { DialogActionTrigger, DialogBody, DialogContent, DialogFooter, DialogHeader, DialogRoot, DialogTitle, DialogTrigger } from "./ui/dialog";
import { TimelineConnector, TimelineContent, TimelineDescription, TimelineItem, TimelineRoot, TimelineTitle } from "./ui/timeline";
import { RevisionsComparisonResult } from "../models/revision_merge";


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

const comparisonResult: RevisionsComparisonResult = {
    divergences: [
        {
            index: 6,
            existingRevisionHash: null,
            upcomingRevisionHash: 'e7719544c0ff396e3edc1dda2b784d44f03ceb73410a471f5b37091f5e43be19d0cb654045906ef6b361b499a771d88f56067acb4afa6c0949c384c421f8e51e'
        }
    ],
    mergedArray: [
        'e3839fff23f468300b65a9be15a452aa160c1ccbe91d657d2d73767100711bb0e97f29fb7949de6aced5a73d1278e6227b8b20225050025fd6af6b8cb6ebb25f',
        'd32a796eb95848ffc2efbc83466e94d838218d4733d41b4f15ce134d443cea4b05b7395674a6d9926b60056cf776e34ea292302a5885606fb1064d5ff5014ad1',
        '2a3d96625db20c3a64b884a41a90af24716e177365647b25163fefd71b85cb285543b0450b1faf8edf93dff69de5d71adc4ee8adf999eb4c4ad3cec7d61973f0',
        '20a2a9ba0178a4a8d8d4f251440e71ac1d8f958c518b5eb6d7be020d58d04ef9640cb69c2e65d33efc736df976024b3663a3684f0bd4b85674fa80c5944b65c4',
        '911c4d27936f6641213a6368541662fdb57ef600c09a7e574be3beb00b56449dcf698e5385af80ac6abbf4b2a5ff0f38c75bdf3ceda6556dbadf3fe4a0341cef',
        '4ebd035c34329a227e4962c4b16ff77db0c380c6e8a1de232df5b1fd956165c826e36180d799039274bac3db8c5be5f583ac04acb38abc29e9f51c2efb4bf8a0',
        'e7719544c0ff396e3edc1dda2b784d44f03ceb73410a471f5b37091f5e43be19d0cb654045906ef6b361b499a771d88f56067acb4afa6c0949c384c421f8e51e'
    ],
    identical: false,
    sameLength: false,
    existingRevisionsLength: 6,
    upcomingRevisionsLength: 7
};

export const ImportAquaChainFromChain = ({ fileInfo, isVerificationSuccessful }: ImportChainFromChainProps) => {

    const [uploading, setUploading] = useState(false)
    const [_uploaded, setUploaded] = useState(false)
    const [dbFiles, setDbFiles] = useState<ApiFileInfo[]>([])

    const { metamaskAddress, setFiles, files, user_profile, backend_url } = useStore(appStore)

    let navigate = useNavigate();

    const importAquaChain = async () => {

        // const existingChainFile = dbFiles.find(file => file.name === fileInfo.name)
        const chainToImport = JSON.parse(fileInfo.page_data).pages[0]
        const existingChainFile = dbFiles.find(file => JSON.parse(file.page_data).pages[0].genesis_hash === chainToImport.genesis_hash)

        if (existingChainFile) {

            const existingFileRevisions = Object.keys(JSON.parse(existingChainFile.page_data).pages[0].revisions)
            const fileToImportRevisions = Object.keys(chainToImport.revisions)

            const mergeResult = analyzeAndMergeRevisions(existingFileRevisions, fileToImportRevisions)

            console.log(mergeResult)


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

            <DialogRoot open={true}>
                <DialogTrigger asChild>
                    <Button variant="outline" size="sm">
                        Open Dialog
                    </Button>
                </DialogTrigger>
                <DialogContent borderRadius={'lg'}>
                    <DialogHeader>
                        <DialogTitle>Aqua Chain Import</DialogTitle>
                    </DialogHeader>
                    <DialogBody>
                        <TimelineRoot>
                            <TimelineItem colorPalette={isVerificationSuccessful ? 'green' : 'red'}>
                                <TimelineConnector>
                                    <LuCheck />
                                </TimelineConnector>
                                <TimelineContent colorPalette={'gray'}>
                                    <TimelineTitle>Verification status</TimelineTitle>
                                    <TimelineDescription>Verification successful</TimelineDescription>
                                </TimelineContent>
                            </TimelineItem>

                            {
                                true ? (
                                    <>
                                        <TimelineItem colorPalette={'green'}>
                                            <TimelineConnector>
                                                <LuCheck />
                                            </TimelineConnector>
                                            <TimelineContent>
                                                <TimelineTitle textStyle="sm">Chains Identical</TimelineTitle>
                                                <TimelineDescription>Chains are identical</TimelineDescription>
                                            </TimelineContent>
                                        </TimelineItem>

                                        <TimelineItem colorPalette={'blue'}>
                                            <TimelineConnector>
                                                <LuX />
                                            </TimelineConnector>
                                            <TimelineContent>
                                                <TimelineTitle textStyle="sm">Action</TimelineTitle>
                                                <TimelineDescription>No Action</TimelineDescription>
                                            </TimelineContent>
                                        </TimelineItem>
                                    </>
                                ) : null
                            }

                            {
                                true ? (
                                    <>
                                        <TimelineItem colorPalette={'green'}>
                                            <TimelineConnector>
                                                <LuCheck />
                                            </TimelineConnector>
                                            <TimelineContent>
                                                <TimelineTitle textStyle="sm">Chain Difference</TimelineTitle>
                                                <TimelineDescription>Existing Chain is Longer than Upcoming Chain</TimelineDescription>
                                            </TimelineContent>
                                        </TimelineItem>

                                        <TimelineItem colorPalette={'blue'}>
                                            <TimelineConnector>
                                                <LuX />
                                            </TimelineConnector>
                                            <TimelineContent>
                                                <TimelineTitle textStyle="sm">Action</TimelineTitle>
                                                <TimelineDescription>No Action</TimelineDescription>
                                            </TimelineContent>
                                        </TimelineItem>
                                    </>
                                ) : null
                            }

                            {
                                true ? (
                                    <>
                                        <TimelineItem colorPalette={'green'}>
                                            <TimelineConnector>
                                                <LuCheck />
                                            </TimelineConnector>
                                            <TimelineContent>
                                                <TimelineTitle textStyle="sm">Chains Length</TimelineTitle>
                                                <TimelineDescription>Chains are of same Length</TimelineDescription>
                                            </TimelineContent>
                                        </TimelineItem>

                                        <TimelineItem colorPalette={'blue'}>
                                            <TimelineConnector>
                                                <LuX />
                                            </TimelineConnector>
                                            <TimelineContent>
                                                <TimelineTitle textStyle="sm">Action</TimelineTitle>
                                                <TimelineDescription>No Action</TimelineDescription>
                                            </TimelineContent>
                                        </TimelineItem>
                                    </>
                                ) : null
                            }


                            {
                                comparisonResult.divergences.length > 0 ? (
                                    <>
                                        <TimelineItem colorPalette={'red'}>
                                            <TimelineConnector>
                                                <LuX />
                                            </TimelineConnector>
                                            <TimelineContent>
                                                <TimelineTitle textStyle="sm">Chains are Different</TimelineTitle>
                                                <TimelineDescription>Chains have divergencies</TimelineDescription>
                                                <List.Root>
                                                    {
                                                        comparisonResult.divergences.map((diff, i: number) => (
                                                            <List.Item key={`diff_${i}`} fontSize={'sm'}>
                                                                {
                                                                    diff.existingRevisionHash ? (
                                                                        <Group>
                                                                            <Text textDecoration={'line-through'}>
                                                                                {formatCryptoAddress(diff.existingRevisionHash ?? "", 15, 4)}
                                                                            </Text>
                                                                            <LuChevronRight />
                                                                            <Text>
                                                                                {formatCryptoAddress(diff.upcomingRevisionHash ?? "", 15, 4)}
                                                                            </Text>
                                                                        </Group>
                                                                    ) : (
                                                                        <>
                                                                            {formatCryptoAddress(diff.upcomingRevisionHash ?? "", 20, 4)}
                                                                        </>
                                                                    )
                                                                }
                                                            </List.Item>
                                                        ))
                                                    }
                                                </List.Root>
                                            </TimelineContent>
                                        </TimelineItem>

                                        <TimelineItem colorPalette={'info'}>
                                            <TimelineConnector>
                                                <LuCheck />
                                            </TimelineConnector>
                                            <TimelineContent>
                                                <TimelineTitle textStyle="sm">Action</TimelineTitle>
                                                <TimelineDescription>Merge Chains</TimelineDescription>
                                            </TimelineContent>
                                        </TimelineItem>
                                    </>
                                ) : null
                            }



                            {/* <TimelineItem>
                                <TimelineConnector>
                                    <LuPackage />
                                </TimelineConnector>
                                <TimelineContent>
                                    <TimelineTitle textStyle="sm">Order Delivered</TimelineTitle>
                                    <TimelineDescription>20th May 2021, 10:30am</TimelineDescription>
                                </TimelineContent>
                            </TimelineItem> */}
                        </TimelineRoot>
                    </DialogBody>
                    <DialogFooter>
                        <DialogActionTrigger asChild>
                            <Button variant="outline">Cancel</Button>
                        </DialogActionTrigger>
                        <Button>Save</Button>
                    </DialogFooter>
                    <DialogCloseTrigger />
                </DialogContent>
            </DialogRoot>

        </Container >
    )
}
