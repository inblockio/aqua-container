import { useEffect, useState } from "react"
import {
    DrawerActionTrigger,
    DrawerBackdrop,
    DrawerBody,
    DrawerCloseTrigger,
    DrawerContent,
    DrawerFooter,
    DrawerHeader,
    DrawerRoot,
    DrawerTitle,

} from "../drawer"
import { Button } from "../button"
import { LuCheck, LuChevronDown, LuChevronUp, LuExternalLink, LuEye, LuX } from "react-icons/lu"
import { Box, Card, Collapsible, For, Group, Icon, Link, Spacer, Span, Text, VStack } from "@chakra-ui/react"
import { TimelineConnector, TimelineContent, TimelineDescription, TimelineItem, TimelineRoot, TimelineTitle } from "../timeline"
import { PageData, Revision } from "../../../models/PageData"
import { formatCryptoAddress, getLastRevisionVerificationHash, timeToHumanFriendly } from "../../../utils/functions"
import { Alert } from "../alert"
import { ClipboardIconButton, ClipboardRoot } from "../clipboard"
import AquaVerifier, { RevisionAquaChainResult, RevisionVerificationResult } from "aqua-verifier";
import ReactLoading from "react-loading"
import { WITNESS_NETWORK_MAP } from "../../../utils/constants"
import { DownloadAquaChain, WitnessAquaChain, SignAquaChain, DeleteAquaChain } from "../../aqua_chain_actions"
import { ApiFileInfo } from "../../../models/FileInfo"
import FilePreview from "../../FilePreview"
import { useStore } from "zustand"
import appStore from "../../../store"


interface IItemDetail {
    label: string
    value: string
    displayValue: string
    showCopyIcon: boolean
}

const ItemDetail = ({ label, value, displayValue, showCopyIcon }: IItemDetail) => {

    return (
        <Group>
            <Text>{label}</Text>
            <Group>
                <Text fontFamily={"monospace"} textWrap={'wrap'} wordBreak={'break-word'}>{displayValue}</Text>
                <ClipboardRoot value={value} hidden={!showCopyIcon}>
                    <ClipboardIconButton size={'xs'} />
                </ClipboardRoot>
            </Group>
        </Group>
    )
}

interface IRevisionDisplay {
    revision: Revision
    verificationResult?: RevisionVerificationResult
}

const RevisionDisplay = ({ revision, verificationResult }: IRevisionDisplay) => {

    const loaderSize = '40px'

    return (
        <div>
            <TimelineItem>
                <TimelineConnector
                    bg={verificationResult?.successful ? "green" : "red"}
                    color={"white"}
                >
                    <Icon fontSize="xs" color={'white'} border={'none'}>
                        {
                            !verificationResult ? (
                                <ReactLoading type={'spin'} color={'blue'} height={loaderSize} width={loaderSize} />
                            ) : (
                                verificationResult.successful ? <LuCheck /> : <LuX />
                            )
                        }
                    </Icon>
                </TimelineConnector>
                <TimelineContent gap="4">
                    <TimelineTitle>
                        <Span>Revision: </Span>
                        <Span color="fg.muted" fontFamily={'monospace'}>{formatCryptoAddress(revision.metadata.verification_hash, 4, 10)}</Span>
                    </TimelineTitle>
                    <Card.Root size="sm">

                        <Card.Body textStyle="sm" lineHeight="tall">
                            <TimelineRoot size="lg" variant="subtle" maxW="md">

                                <TimelineItem>
                                    <TimelineConnector
                                        bg={verificationResult?.content_verification.successful ? "green" : "red"}
                                    >
                                        <Icon fontSize="xs" color={'white'}>
                                            {
                                                verificationResult?.content_verification.successful ? <LuCheck /> :
                                                    <LuX />
                                            }
                                        </Icon>
                                    </TimelineConnector>
                                    <TimelineContent gap="2">
                                        <TimelineTitle>
                                            <Span>
                                                Revision content is
                                                {verificationResult?.content_verification.successful ? ' valid' : ' invalid'}
                                            </Span>
                                        </TimelineTitle>
                                    </TimelineContent>
                                </TimelineItem>

                                <TimelineItem>
                                    <TimelineConnector
                                        bg={verificationResult?.metadata_verification.successful ? "green" : "red"}
                                    >
                                        <Icon fontSize="xs" color={'white'}>
                                            {
                                                verificationResult?.metadata_verification.successful ? <LuCheck /> :
                                                    <LuX />
                                            }
                                        </Icon>
                                    </TimelineConnector>
                                    <TimelineContent gap="2">
                                        <TimelineTitle>
                                            <Span>
                                                Revision metadata is
                                                {verificationResult?.metadata_verification.successful ? ' valid' : ' invalid'}
                                            </Span>
                                        </TimelineTitle>
                                        <TimelineDescription>{timeToHumanFriendly(revision.metadata.time_stamp, true)}</TimelineDescription>
                                        <ItemDetail label="Metadata Hash:"
                                            displayValue={formatCryptoAddress(revision.metadata.metadata_hash, 4, 6)}
                                            value={revision.metadata.metadata_hash} showCopyIcon={true}
                                        />
                                    </TimelineContent>
                                </TimelineItem>

                                {
                                    revision.signature ? (
                                        <TimelineItem>
                                            <TimelineConnector
                                                bg={verificationResult?.signature_verification.successful ? "green" : "red"}
                                            >
                                                <Icon fontSize="xs" color={'white'}>
                                                    {
                                                        verificationResult?.signature_verification.successful ? <LuCheck /> :
                                                            <LuX />
                                                    }
                                                </Icon>
                                            </TimelineConnector>
                                            <TimelineContent gap="2">
                                                <TimelineTitle>
                                                    <Span>
                                                        Revision signature is
                                                        {verificationResult?.signature_verification.successful ? ' valid' : ' invalid'}
                                                    </Span>
                                                </TimelineTitle>
                                                <ItemDetail label="Signature:"
                                                    displayValue={formatCryptoAddress(revision.signature.signature, 4, 6)}
                                                    value={revision.signature.signature} showCopyIcon={true}
                                                />
                                                <ItemDetail label="Signature Hash:"
                                                    displayValue={formatCryptoAddress(revision.signature.signature_hash, 4, 6)}
                                                    value={revision.signature.signature_hash} showCopyIcon={true}
                                                />
                                                <ItemDetail label="Wallet Address:"
                                                    // displayValue={formatCryptoAddress(revision.signature.wallet_address, 4, 6)}
                                                    displayValue={revision.signature.wallet_address}
                                                    value={revision.signature.wallet_address} showCopyIcon={true}
                                                />
                                                <ItemDetail label="Public Key:"
                                                    displayValue={formatCryptoAddress(revision.signature.public_key, 4, 6)}
                                                    value={revision.signature.public_key} showCopyIcon={true}
                                                />
                                            </TimelineContent>
                                        </TimelineItem>
                                    ) : (

                                        <TimelineItem>
                                            <TimelineConnector
                                                bg={'gray.400'}
                                            >
                                                <Icon fontSize="xs" color={'white'}>
                                                    <LuX />
                                                </Icon>
                                            </TimelineConnector>
                                            <TimelineContent gap="2">
                                                <TimelineTitle>
                                                    <Span>Revision has no signature</Span>
                                                </TimelineTitle>
                                            </TimelineContent>
                                        </TimelineItem>

                                    )
                                }

                                {
                                    revision.witness ? (
                                        <TimelineItem>
                                            <TimelineConnector
                                                bg={verificationResult?.witness_verification.successful ? "green" : "red"}
                                            >
                                                <Icon fontSize="xs" color={'white'}>
                                                    {
                                                        verificationResult?.witness_verification.successful ? <LuCheck /> :
                                                            <LuX />
                                                    }
                                                </Icon>
                                            </TimelineConnector>
                                            <TimelineContent gap="2">
                                                <TimelineTitle>
                                                    <Span>
                                                        Revision witness is
                                                        {verificationResult?.witness_verification.successful ? ' valid' : ' invalid'}
                                                    </Span>
                                                </TimelineTitle>
                                                <ItemDetail label="Domain snapshot Hash:"
                                                    displayValue={formatCryptoAddress(revision.witness.domain_snapshot_genesis_hash, 4, 6)}
                                                    value={revision.witness.domain_snapshot_genesis_hash} showCopyIcon={true}
                                                />
                                                <ItemDetail label="Network:"
                                                    displayValue={formatCryptoAddress(revision.witness.witness_network, 4, 6)}
                                                    value={revision.witness.witness_network} showCopyIcon={false}
                                                />
                                                <ItemDetail label="Witness Hash:"
                                                    displayValue={formatCryptoAddress(revision.witness.witness_hash, 4, 6)}
                                                    value={revision.witness.witness_hash} showCopyIcon={true}
                                                />
                                                <Group>
                                                    <ItemDetail label="Transaction Hash:"
                                                        displayValue={formatCryptoAddress(revision.witness.witness_event_transaction_hash.startsWith('0x') ? revision.witness.witness_event_transaction_hash : `0x${revision.witness.witness_event_transaction_hash}`, 4, 6)}
                                                        value={`0x${revision.witness.witness_event_transaction_hash}`} showCopyIcon={true}
                                                    />
                                                    <Link outline={'none'} href={`${WITNESS_NETWORK_MAP[revision.witness.witness_network]}/${revision.witness.witness_event_transaction_hash}`} target="_blank">
                                                        <Icon size={'lg'} color={'blue.500'}>
                                                            <LuExternalLink />
                                                        </Icon>
                                                    </Link>
                                                </Group>
                                                <ItemDetail label="Verification Hash:"
                                                    displayValue={formatCryptoAddress(revision.witness.witness_event_verification_hash, 4, 6)}
                                                    value={revision.witness.witness_event_verification_hash} showCopyIcon={true}
                                                />
                                            </TimelineContent>
                                        </TimelineItem>
                                    ) : (

                                        <TimelineItem>
                                            <TimelineConnector
                                                bg={'gray.400'}
                                            >
                                                <Icon fontSize="xs" color={'white'}>
                                                    <LuX />
                                                </Icon>
                                            </TimelineConnector>
                                            <TimelineContent gap="2">
                                                <TimelineTitle>
                                                    <Span>Revision has no witness</Span>
                                                </TimelineTitle>
                                            </TimelineContent>
                                        </TimelineItem>

                                    )
                                }

                            </TimelineRoot>

                        </Card.Body>
                        <Card.Footer>
                            <Alert status={verificationResult?.successful ? 'success' : 'error'} title={verificationResult?.successful ? "This revision is valid" : "This revision is invalid"} />
                        </Card.Footer>
                    </Card.Root>
                </TimelineContent>
            </TimelineItem>
        </div>
    )
}

interface IPageDataDetails {
    fileInfo: ApiFileInfo
    callBack?: (res: boolean) => void
}

const ChainDetails = ({ fileInfo, callBack }: IPageDataDetails) => {
    const [verificationResult, setVerificationResult] = useState<RevisionAquaChainResult | null>(null)
    const pageData: PageData = JSON.parse(fileInfo.page_data)


    const verifyAquaChain = () => {

        const verifier = new AquaVerifier({
            alchemyKey: 'ZaQtnup49WhU7fxrujVpkFdRz4JaFRtZ',
            doAlchemyKeyLookUp: true,
            version: 1.2
        });

        verifier.verifyAquaChain(pageData.pages[0]).then((res) => {
            setVerificationResult(res)
            callBack && callBack(res.successful)
        }).catch((error: any) => {
            console.error("Failed to verify aqua chain: ", error)
        }).finally(() => {
            console.info("Verification complete")
        })
    }

    useEffect(() => {
        if (pageData) {
            verifyAquaChain()
        }
    }, [])

    return (
        <>
            <TimelineRoot size="lg" variant="subtle" maxW="xl">
                <For
                    each={Object.values(pageData.pages[0].revisions)}
                >
                    {(revision, index) => (
                        <RevisionDisplay key={`revision_${index}`} revision={revision} verificationResult={verificationResult?.revisionResults[index]} />
                    )}
                </For>
            </TimelineRoot>
        </>
    )
}

export const ChainDetailsBtn = ({ fileInfo }: IPageDataDetails) => {

    const [showMoreDetails, setShowMoreDetails] = useState(false)

    const { backend_url } = useStore(appStore)
    const [isOpen, setIsOpen] = useState(false)
    const pageData: PageData = JSON.parse(fileInfo.page_data)
    const [isVerificationSuccessful, setIsVerificationSuccessful] = useState<boolean>(false)
    const [lastVerificationHash, setLastVerificationHash] = useState<string | null>(null)


    const updateVerificationStatus = (result: boolean) => {
        setIsVerificationSuccessful(result)
    }

    useEffect(() => {
        const hash = getLastRevisionVerificationHash(pageData)
        setLastVerificationHash(hash)
        console.log("ChainDetailsBtn == > " + JSON.stringify(fileInfo))
    }, [fileInfo])


    return (
        <>
            <Button size={'xs'} colorPalette={'green'} variant={'subtle'} w={'80px'} onClick={() => setIsOpen(true)}>
                <LuEye />
                Details
            </Button>

            <DrawerRoot open={isOpen} size={{ base: 'full', md: 'lg' }} onOpenChange={(e) => setIsOpen(e.open)}>
                <DrawerBackdrop />
                {/* <DrawerTrigger asChild>
                    <Button size={'xs'} colorPalette={'green'} variant={'subtle'} w={'80px'}>
                        <LuEye />
                        Details
                    </Button>
                </DrawerTrigger> */}
                <DrawerContent borderLeftRadius={'xl'} overflow={'hidden'}>
                    <DrawerHeader bg={{ base: isVerificationSuccessful ? 'green.100' : 'red.100', _dark: isVerificationSuccessful ? 'green.900' : 'red.900' }}>
                        <DrawerTitle>{pageData?.pages[0]?.title}</DrawerTitle>
                    </DrawerHeader>
                    <DrawerBody py={'lg'} px={1}>
                        <Card.Root border={'none'} shadow={'md'} borderRadius={'xl'}>
                            <Card.Body>
                                <FilePreview fileInfo={fileInfo} />
                            </Card.Body>
                        </Card.Root>
                        <Spacer height={'20px'} />

                        <Card.Root borderRadius={'lg'}>
                            <Card.Body>
                                <VStack gap={'4'}>
                                    <Alert status={isVerificationSuccessful ? 'success' : 'error'} title={isVerificationSuccessful ? "This chain is valid" : "This chain is invalid"} />
                                    <Box w={'100%'}>
                                        <Collapsible.Root open={showMoreDetails}>
                                            <Collapsible.Trigger w="100%" py={'md'} onClick={() => setShowMoreDetails(open => !open)} cursor={'pointer'}>
                                                <Alert w={'100%'} status={"info"} textAlign={'start'} title={showMoreDetails ? `Show less Details` : `Show more Details`} icon={showMoreDetails ? <LuChevronUp /> : <LuChevronDown />} />
                                            </Collapsible.Trigger>
                                            <Collapsible.Content py={'4'}>
                                                <ChainDetails fileInfo={fileInfo} callBack={updateVerificationStatus} />
                                            </Collapsible.Content>
                                        </Collapsible.Root>
                                    </Box>
                                    <Box minH={'400px'} />
                                </VStack>
                            </Card.Body>
                        </Card.Root>

                    </DrawerBody>
                    <DrawerFooter flexWrap={'wrap'}>
                        <DrawerActionTrigger asChild>
                            <Button variant="outline" size={'sm'}>Close</Button>
                        </DrawerActionTrigger>
                        <DownloadAquaChain file={fileInfo} />
                        <WitnessAquaChain backend_url={backend_url} file_id={fileInfo.id} filename={fileInfo.name} lastRevisionVerificationHash={lastVerificationHash ?? ""} />
                        <SignAquaChain backend_url={backend_url} file_id={fileInfo.id} filename={fileInfo.name} lastRevisionVerificationHash={lastVerificationHash ?? ""} />
                        <DeleteAquaChain backend_url={backend_url} file_id={fileInfo.id} filename={fileInfo.name} />
                    </DrawerFooter>
                    <DrawerCloseTrigger />
                </DrawerContent>
            </DrawerRoot>

        </>
    )
}

export default ChainDetails