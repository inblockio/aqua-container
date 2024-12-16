import { useEffect, useState } from 'react'
import { useParams } from 'react-router-dom'
import { useStore } from 'zustand'
import appStore from '../store'
import axios from 'axios'
import { ApiFileInfo } from '../models/FileInfo'
import { toaster } from '../components/ui/toaster'
import Loading from 'react-loading'
import { Box, Card, Center, Container, Group, VStack } from '@chakra-ui/react'
import ChainDetails from '../components/ui/navigation/CustomDrawer'
import FilePreview from '../components/FilePreview'
import ConnectWallet from '../components/ui/navigation/ConnectWallet'
import { ImportAquaChainFromChain } from '../components/dropzone_file_actions'
import { Alert } from '../components/ui/alert'

const SharePage = () => {
    const { backend_url, metamaskAddress } = useStore(appStore)
    const [fileInfo, setFileInfo] = useState<ApiFileInfo | null>(null)
    const [loading, setLoading] = useState(false)
    const [hasError, setHasError] = useState<string | null>(null);
    const [isVerificationSuccesful, setIsVerificationSuccessful] = useState(false)

    const params = useParams()

    const loadPageData = async () => {
        if (!backend_url.includes('0.0.0.0')) {
            try {

                setLoading(true)
                const url = `${backend_url}/share_data/${params.identifier}`;
                console.log("url is ", url)
                const response = await axios.get(url, {
                    headers: {
                        'Content-Type': 'application/x-www-form-urlencoded'
                    }
                });
                console.log(response)

                if (response.status === 200) {
                    setFileInfo(response.data.file_data)
                }
                setLoading(false)
            }
            catch (error: any) {
                if (error.response.status == 404){
                    setHasError(`File could not be found (probably it was deleted)`);
                }else{
                    setHasError(`Error : ${error}`);
                }
                console.error(error) ;

                
               
                toaster.create({
                    description: `Error fetching data`,
                    type: 'error'
                });
            }
        }
    }

    useEffect(() => {
        if (params.identifier) {
            loadPageData()
        }
    }, [params, metamaskAddress, backend_url])

    useEffect(() => {
        // if (!metamaskAddress) {
        //     toaster.create({
        //         description: "Sign In is required",
        //         type: "info"
        //     })
        // }
    }, [])

   const  showProperWidget = () => {
        if (hasError) {
            return <Center>
                <Alert status="error" title="An error occured">
                    {hasError}
                </Alert>
            </Center>
        }
        if (loading) {
            return <Center>
                <Loading type='spin' width={'80px'} />
            </Center>
        }
        return <div />
    }
    return (
        <div>
            {
                showProperWidget()
            }
            {
                fileInfo ? (
                    <Container>
                        <VStack gap={'10'}>
                            <Group justifyContent={'center'} w={'100%'}>
                                {
                                    !metamaskAddress ? (
                                        <ConnectWallet />
                                    ) : (
                                        <ImportAquaChainFromChain fileInfo={fileInfo} isVerificationSuccessful={isVerificationSuccesful} />
                                    )
                                }
                            </Group>
                            <Box w={'100%'}>
                                <Card.Root border={'none'} shadow={'md'} borderRadius={'xl'}>
                                    <Card.Body>
                                        <FilePreview fileInfo={fileInfo} />
                                    </Card.Body>
                                </Card.Root>
                            </Box>
                            <Box w={'100%'}>
                                <ChainDetails fileInfo={fileInfo} callBack={(res) => setIsVerificationSuccessful(res)} />
                            </Box>
                        </VStack>
                    </Container>
                ) : null
            }
        </div>
    )
}

export default SharePage