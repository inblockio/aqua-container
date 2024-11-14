import { useState } from 'react'
import { Button } from "../button"
import {
    DialogBody,
    DialogCloseTrigger,
    DialogContent,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from "../dialog"
import { Dialog, Text, VStack } from '@chakra-ui/react'
import { LuCheckCircle2, LuWallet2, LuXCircle } from 'react-icons/lu'
import ReactLoading from 'react-loading'
import { fetchFiles, formatCryptoAddress, remove0xPrefix, setCookie } from '../../../utils/functions'
import { SiweMessage, generateNonce } from 'siwe'
import { ENDPOINTS } from '../../../utils/constants'
import axios from 'axios'
import { useStore } from 'zustand'
import appStore from '../../../store'
import { BrowserProvider } from 'ethers'

export default function ConnectWallet() {

    const { metamaskAddress, setMetamaskAddress, setFiles } = useStore(appStore);

    const [isOpen, setIsOpen] = useState(false)
    const [connectionState, setConnectionState] = useState<'idle' | 'connecting' | 'success' | 'error'>('idle')
    const [_progress, setProgress] = useState(0)

    const iconSize = "120px"

    const resetState = () => {
        setConnectionState('idle')
        setProgress(0)
    }

    function createSiweMessage(address: string, statement: string) {

        // const scheme = window.location.protocol.slice(0, -1);
        const domain = window.location.host;
        const origin = window.location.origin;
        const expiry = new Date(Date.now() + 24 * 60 * 60 * 1000).toISOString()
        console.log("expiry: ", expiry,)
        const message = new SiweMessage({
            // Setting scheme is giving out lots of headaches
            // scheme: scheme,
            domain,
            address,
            statement,
            uri: origin,
            version: '1',
            chainId: 2,
            nonce: generateNonce(),
            expirationTime: expiry,
            issuedAt: new Date(Date.now()).toISOString()
        });
        return message.prepareMessage();
    }

    const signAndConnect = async () => {
        if (window.ethereum) {
            setConnectionState('connecting')
            const provider = new BrowserProvider(window.ethereum);
            try {
                // Connect wallet
                await window.ethereum.request({ method: 'eth_requestAccounts' });
                const signer = await provider.getSigner();

                // Create a SIWE msg for signing
                const message = createSiweMessage(
                    signer.address,
                    'Sign in with Ethereum to the app.'
                );

                let signature = await signer.signMessage(message);

                const formData = new URLSearchParams();

                formData.append('message', message);
                formData.append('signature', remove0xPrefix(signature));

                const response = await axios.post(ENDPOINTS.SIWE_SIGN_IN, formData, {
                    headers: {
                        'Content-Type': 'application/x-www-form-urlencoded'
                    }
                });

                if (response.status === 200) {
                    if (signature) {
                        const responseData = response.data
                        const walletAddress = responseData?.session?.address
                        setMetamaskAddress(walletAddress)

                        const expirationDate = new Date(responseData?.session?.expiration_time);
                        setCookie('pkc_nonce', `${responseData.session.nonce}`, expirationDate)
                        setConnectionState('success')

                        let files = await fetchFiles(walletAddress);
                        setFiles(files)
                    }
                }
                setTimeout(() => {
                    setIsOpen(false)
                    resetState()
                }, 2000)

            } catch (error: any) {
                setConnectionState("error")
                console.error('Error during wallet connection or signing:', error);
            }
        } else {
            alert('MetaMask is not installed');
        }
    };

    return (
        <Dialog.Root placement={'center'} size={'sm'} open={isOpen} onOpenChange={(details) => setIsOpen(details.open)}>
            <DialogTrigger asChild>
                <Button size={'sm'} borderRadius={'md'} onClick={() => {
                    setIsOpen(true)
                    signAndConnect()
                }}>
                    <LuWallet2 />
                    {metamaskAddress ? formatCryptoAddress(metamaskAddress, 3, 3) : 'Sign In'}
                </Button>
            </DialogTrigger>
            <DialogContent borderRadius={'2xl'}>
                <DialogHeader>
                    <DialogTitle fontWeight={500} color={'gray.800'} _dark={{ color: 'white' }}>Wallet Connection</DialogTitle>
                </DialogHeader>
                <DialogBody >
                    <VStack gap={'10'}>
                        {connectionState === 'connecting' && (
                            <>
                                <ReactLoading type={'spin'} color={'blue'} height={iconSize} width={iconSize} />
                                <Text fontSize={'md'}>Connecting to wallet...</Text>
                            </>
                        )}
                        {connectionState === 'success' && (
                            <>
                                <LuCheckCircle2 strokeWidth='1px' color='green' size={iconSize} />
                                <Text fontSize={'md'} color={'green.700'}>Successfully connected!</Text>
                            </>
                        )}
                        {connectionState === 'error' && (
                            <>
                                <LuXCircle color='red' strokeWidth='1px' size={iconSize} />
                                <Text fontSize={'md'} color={'red.700'}>Error connecting to wallet</Text>
                            </>
                        )}
                    </VStack>
                </DialogBody>
                <DialogCloseTrigger />
            </DialogContent>
        </Dialog.Root>
    )
}