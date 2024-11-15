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
import { LuCheck, LuEye, LuFileSignature, LuGlasses, LuShare2, LuX } from "react-icons/lu"
import { Card, For, Group, Icon, Span, Text } from "@chakra-ui/react"
import { TimelineConnector, TimelineContent, TimelineDescription, TimelineItem, TimelineRoot, TimelineTitle } from "../timeline"
import { PageData, Revision } from "../../../models/PageData"
import { formatCryptoAddress, timeToHumanFriendly } from "../../../utils/functions"
import { Alert } from "../alert"
import { ClipboardIconButton, ClipboardRoot } from "../clipboard"
import AquaVerifier, { RevisionAquaChainResult, RevisionVerificationResult } from "aqua-verifier";
import ReactLoading from "react-loading"


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
        <>
            <TimelineItem>
                <TimelineConnector
                    bg={verificationResult?.successful ? "green" : "red"}
                    color={"white"}
                >
                    <Icon fontSize="xs" color={'white'}>
                        {
                            !verificationResult ? (
                                <ReactLoading type={'spin'} color={'blue'} height={loaderSize} width={loaderSize} />
                            ) : (
                                <>
                                    {
                                        verificationResult.successful ? <LuCheck /> :
                                            <LuX />
                                    }
                                </>
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
                                                <ItemDetail label="Signature Hash:"
                                                    displayValue={formatCryptoAddress(revision.signature.wallet_address, 4, 6)}
                                                    value={revision.signature.wallet_address} showCopyIcon={true}
                                                />
                                                <ItemDetail label="Signature Hash:"
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
                                                <ItemDetail label="Transaction Hash:"
                                                    displayValue={formatCryptoAddress(revision.witness.witness_event_transaction_hash.startsWith('0x') ? revision.witness.witness_event_transaction_hash : `0x${revision.witness.witness_event_transaction_hash}`, 4, 6)}
                                                    value={`0x${revision.witness.witness_event_transaction_hash}`} showCopyIcon={true}
                                                />
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
        </>
    )
}

interface IPageDataDetails {
    pageData: PageData
}

const ChainDetails = ({ pageData }: IPageDataDetails) => {

    const [open, setOpen] = useState(false)
    const [verificationResult, setVerificationResult] = useState<RevisionAquaChainResult | null>(null)

    const verifyAquaChain = () => {

        let verifier = new AquaVerifier({
            alchemyKey: 'ZaQtnup49WhU7fxrujVpkFdRz4JaFRtZ',
            doAlchemyKeyLookUp: true,
            version: 1.2
        });
        console.log("Verification started")
        verifier.verifyAquaChain(pageData.pages[0]).then((res) => {
            setVerificationResult(res)
        }).catch((error: any) => {
            console.error("Failed to verify aqua chain: ", error)
        }).finally(() => {
            console.info("Verification complete")
        })
    }

    useEffect(() => {
        if (open) {
            console.info("Verification Trigger pulled")
            verifyAquaChain()
        }
    }, [open])

    return (
        <DrawerRoot open={open} size={{ base: 'full', md: 'xl' }} onOpenChange={(e) => setOpen(e.open)}>
            <DrawerBackdrop />
            <DrawerTrigger asChild>
                <Button size={'xs'} colorPalette={'green'} variant={'subtle'} w={'80px'}>
                    <LuEye />
                    Details
                </Button>
            </DrawerTrigger>
            <DrawerContent>
                <DrawerHeader bg={'red.100'}>
                    <DrawerTitle>FileName.json</DrawerTitle>
                </DrawerHeader>
                <DrawerBody py={'lg'}>
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
                    <Button size={'sm'}>
                        <LuShare2 />
                        Share
                    </Button>
                    <Button size={'sm'}>
                        <LuGlasses />
                        Witness
                    </Button>
                    <Button size={'sm'}>
                        <LuFileSignature />
                        Sign
                    </Button>
                </DrawerFooter>
                <DrawerCloseTrigger />
            </DrawerContent>
        </DrawerRoot>
    )
}

export default ChainDetails