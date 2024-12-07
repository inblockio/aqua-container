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
import { Center, Dialog, Text, VStack } from '@chakra-ui/react'
import { LuCheckCircle2, LuLogOut, LuWallet2, LuXCircle } from 'react-icons/lu'
import ReactLoading from 'react-loading'
import { fetchFiles, formatCryptoAddress, generateAvatar, remove0xPrefix, setCookie } from '../../../utils/functions'
import { SiweMessage, generateNonce } from 'siwe'
import { ENDPOINTS, SESSION_COOKIE_NAME } from '../../../utils/constants'
import axios from 'axios'
import { useStore } from 'zustand'
import appStore from '../../../store'
import { BrowserProvider, ethers } from 'ethers'
import { Avatar } from '../avatar'
import { toaster } from '../toaster'

export default function ConnectWallet() {

    const { metamaskAddress, setMetamaskAddress, setFiles, avatar, setAvatar } = useStore(appStore);

    const [isOpen, setIsOpen] = useState(false)
    const [loading, setLoading] = useState(false)
    const [connectionState, setConnectionState] = useState<'idle' | 'connecting' | 'success' | 'error'>('idle')
    // const [avatar, setAvatar] = useState("")
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
            setLoading(true)
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
                        const walletAddress = ethers.getAddress(responseData?.session?.address)
                        setMetamaskAddress(walletAddress)
                        let avatar = generateAvatar(walletAddress)
                        setAvatar(avatar)
                        const expirationDate = new Date(responseData?.session?.expiration_time);
                        setCookie(SESSION_COOKIE_NAME, `${responseData.session.nonce}`, expirationDate)
                        setConnectionState('success')

                        let files = await fetchFiles(walletAddress);
                        setFiles(files)
                    }
                }
                setLoading(false)
                toaster.create({
                    description: "Sign In successful",
                    type: "success"
                })
                setTimeout(() => {
                    setIsOpen(false)
                    resetState()
                    setLoading(false)
                }, 2000)

            } catch (error: any) {
                setConnectionState("error")
                setLoading(false)
                console.error('Error during wallet connection or signing:', error);
            }
        } else {
            alert('MetaMask is not installed');
        }
    };

    const signOut = () => {
        setLoading(true)
        setCookie(SESSION_COOKIE_NAME, '', new Date('1970-01-01T00:00:00Z'));
        setMetamaskAddress(null)
        setAvatar(undefined)
        setLoading(false)
        setIsOpen(false)
        toaster.create({
            description: "Signed out successfully",
            type: "success"
        })
    }

    return (
        <Dialog.Root placement={'center'} size={'sm'} open={isOpen} onOpenChange={(details) => setIsOpen(details.open)}>
            <DialogTrigger asChild>
                <Button size={'sm'} borderRadius={'md'} onClick={() => {
                    setIsOpen(true)
                    !metamaskAddress && signAndConnect()
                }}>
                    <LuWallet2 />
                    {metamaskAddress ? formatCryptoAddress(metamaskAddress, 3, 3) : 'Sign In'}
                </Button>
            </DialogTrigger>
            <DialogContent borderRadius={'2xl'} overflow={"hidden"}>
                <DialogHeader py={'3'} px={"5"} bg={{base: 'rgb(188 220 255 / 22%)', _dark: 'rgba(0, 0, 0, 0.3)'}}>
                    <DialogTitle fontWeight={500} color={'gray.800'} _dark={{ color: 'white' }}>
                        {
                            metamaskAddress ? "Account" : "Sign In"
                        }
                    </DialogTitle>
                </DialogHeader>
                <DialogBody py={'8'} px={"5"}>
                    {
                        metamaskAddress ? (
                            <VStack gap={5}>
                                <Center>
                                    <Avatar src={avatar} size={'2xl'} loading='eager' />
                                </Center>
                                <Text fontFamily={'monospace'}>
                                    {formatCryptoAddress(metamaskAddress, 10, 10)}
                                </Text>
                                <Button borderRadius={'md'} loading={loading} onClick={signOut}>
                                    Sign Out
                                    <LuLogOut />
                                </Button>
                            </VStack>
                        ) : (
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
                        )
                    }
                </DialogBody>
                <DialogCloseTrigger />
            </DialogContent>
        </Dialog.Root >
    )
}