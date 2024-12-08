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
    DrawerTrigger,
} from "../drawer"
import { Button } from "../button"
import { LuCheck, LuExternalLink, LuEye, LuX } from "react-icons/lu"
import { Card, For, Group, Icon, Link, Spacer, Span, Text } from "@chakra-ui/react"
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
                <Text fontFamily={"monospace"}>{displayValue}</Text>
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
                                        <TimelineDescription>{timeToHumanFriendly(revision.metadata.time_stamp)}</TimelineDescription>
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
                                                        Revision content is
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
}

const ChainDetails = ({ fileInfo }: IPageDataDetails) => {

    const {  backend_url } = useStore(appStore)
    const [open, setOpen] = useState(false)
    const [verificationResult, setVerificationResult] = useState<RevisionAquaChainResult | null>(null)
    const pageData: PageData = JSON.parse(fileInfo.page_data)

    const file: ApiFileInfo = {
        id: 0,
        name: pageData.pages[0].title,
        extension: "",
        page_data: JSON.stringify(pageData),
        mode: "",
        owner: ""
    }

    const verifyAquaChain = () => {

        let verifier = new AquaVerifier({
            alchemyKey: 'ZaQtnup49WhU7fxrujVpkFdRz4JaFRtZ',
            doAlchemyKeyLookUp: true,
            version: 1.2
        });

        verifier.verifyAquaChain(pageData.pages[0]).then((res) => {
            setVerificationResult(res)
        }).catch((error: any) => {
            console.error("Failed to verify aqua chain: ", error)
        }).finally(() => {
            console.info("Verification complete")
        })
    }

    useEffect(() => {
        if (open && pageData) {
            verifyAquaChain()
        }
    }, [open])

    useEffect(() => {
        if (!pageData) {
            setOpen(false)
        }
    }, [pageData])

    return (
        <DrawerRoot open={open} size={{ base: 'full', md: 'lg' }} onOpenChange={(e) => setOpen(e.open)}>
            <DrawerBackdrop />
            <DrawerTrigger asChild>
                <Button size={'xs'} colorPalette={'green'} variant={'subtle'} w={'80px'}>
                    <LuEye />
                    Details
                </Button>
            </DrawerTrigger>
            <DrawerContent borderLeftRadius={'xl'} overflow={'hidden'}>
                <DrawerHeader bg={{ base: verificationResult ? verificationResult?.successful ? 'green.100' : 'red.100' : 'rgb(188 220 255 / 22%)', _dark: verificationResult ? verificationResult?.successful ? 'green.900' : 'red.900' : 'rgba(0, 0, 0, 0.3)' }}>
                    <DrawerTitle>{pageData?.pages[0]?.title}</DrawerTitle>
                </DrawerHeader>
                <DrawerBody py={'lg'}>
                    <Card.Root border={'none'} shadow={'md'} borderRadius={'xl'}>
                        <Card.Body>
                            <FilePreview fileInfo={fileInfo} />
                        </Card.Body>
                    </Card.Root>
                    <Spacer height={'20px'} />
                    {/* <Text color={'gray.800'} _dark={{ color: 'white' }}>
                        Aqua Chain Details
                    </Text> */}
                    {/* <For
                        each={pageData.pages}
                    >
                        {(hashChain, hashChainIndex) => ( */}
                    <TimelineRoot size="lg" variant="subtle" maxW="xl">
                        <For
                            each={Object.values(pageData.pages[0].revisions)}
                        >
                            {(revision, index) => (
                                <RevisionDisplay key={`revision_${index}`} revision={revision} verificationResult={verificationResult?.revisionResults[index]} />
                            )}
                        </For>
                    </TimelineRoot>
                    {/* )}
                    </For> */}
                </DrawerBody>
                <DrawerFooter>
                    <DrawerActionTrigger asChild>
                        <Button variant="outline" size={'sm'}>Close</Button>
                    </DrawerActionTrigger>
                    <DownloadAquaChain file={file} />
                    <WitnessAquaChain backend_url={backend_url} filename={file.name} lastRevisionVerificationHash={getLastRevisionVerificationHash(pageData)} />
                    <SignAquaChain backend_url={backend_url} filename={file.name} lastRevisionVerificationHash={getLastRevisionVerificationHash(pageData)} />
                    <DeleteAquaChain  backend_url={backend_url} filename={file.name} />
                </DrawerFooter>
                <DrawerCloseTrigger />
            </DrawerContent>
        </DrawerRoot>
    )
}

export default ChainDetails