import { Button } from '@chakra-ui/react'
import React, { useEffect } from 'react'
import { DialogBody, DialogCloseTrigger, DialogContent, DialogHeader, DialogRoot, DialogTitle, DialogTrigger } from './ui/dialog'
import ImportPage from '../pages/ImportPage'
import { ApiFileInfo } from '../models/FileInfo'
import { readJsonFile } from '../utils/functions'

interface IImportByModalProps {
    file: File
    fileIndex: number
    uploadedIndexes: number[]
    updateUploadedIndex: (fileIndex: number) => void
}

const ImportByModal = ({ file }: IImportByModalProps) => {

    // const [verifying, setVerifying] = React.useState(false)
    const [apiFileInfo, setApiFileInfo] = React.useState<ApiFileInfo | undefined>(undefined)

    const handleLoadFile = () => {
            // setVerifying(true)
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
                    setApiFileInfo(hashChain)
                    // const hashChainString = JSON.stringify(hashChain)
                    // console.log("JSON data:", hashChain);
                    // setAppState("selectedFileFromApi", hashChain);
                    // navigate("/details");
                    // Handle the JSON data here
                })
                .catch(() => {
                    // Handle the error here
                });
            // setVerifying(false)
        };
    
        useEffect(() => {
            handleLoadFile()
        }, [])

    return (
        <>
            <DialogRoot size={'xl'}>
                <DialogTrigger asChild>
                    <Button variant="outline" size="sm">
                        Import
                    </Button>
                </DialogTrigger>
                <DialogContent borderRadius={'lg'}>
                    <DialogHeader>
                        <DialogTitle>Import Aqua Chain</DialogTitle>
                    </DialogHeader>
                    <DialogBody>
                        {
                            apiFileInfo ? (
                                <ImportPage incomingFileInfo={apiFileInfo} />
                            ) : null
                        }
                    </DialogBody>
                    <DialogCloseTrigger />
                </DialogContent>
            </DialogRoot>

        </>
    )
}

export default ImportByModal