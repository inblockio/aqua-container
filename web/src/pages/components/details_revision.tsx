import { Component, JSX, createEffect, createSignal } from "solid-js";
import { formatCryptoAddress, timeToHumanFriendly } from "../../util";
import { AquaVerifier, ResultStatus, RevisionVerificationResult } from "aqua-verifier";
import { Revision } from "../../models/PageData";
import { DetailsPageSignature } from "./details_signature";
import { DetailsPageWitness } from "./details_witness";

interface DetailsPageRevisionProps {
    revision: Revision;
    index: number,
    revisionHash: string,
    copyView: (arg1: string, arg2: string) => JSX.Element; 
}

export const DetailsPageRevision: Component<DetailsPageRevisionProps> = (props) => {
    const [isLoading, setIsLoading] = createSignal(false);
    const [result, setResult] = createSignal<RevisionVerificationResult | null>(null);


    createEffect(async () => {
        setIsLoading(true);
        let verifier = new AquaVerifier();
        let res = await verifier.verifyRevision(props.revision)
        setResult(res);
        setIsLoading(false);
    });

    let showRevisionSummary = () => {
        if (result()?.successful) {
            return <div class="border bg-success/10 text-info border-info/20 rounded px-4 py-3 flex justify-between items-center">
                This revision is succesfull
            </div>
        } else {
            return <div class="border bg-danger/10 text-danger border-danger/20 rounded px-4 py-3 flex justify-between items-center">
                This revision is not succesfull
            </div>
        }

    }

    let showRevisionFileVerificationResults = () => {
        if (result()?.file_verification.successful) {
            return <div class="border bg-success/10 text-info border-info/20 rounded px-4 py-3 flex justify-between items-center">
                File revision is valid
            </div>
        } else {
            return <div class="border bg-danger/10 text-danger border-danger/20 rounded px-4 py-3 flex justify-between items-center">
                File revision is not valid {result()?.file_verification.message}
            </div>
        }
    }

    let showRevisionMetadataVerificationResults = () => {
        if (result()?.metadata_verification.successful) {
            return <div class="border bg-success/10 text-info border-info/20 rounded px-4 py-3 flex justify-between items-center">
                 revision metadata is valid
            </div>
        } else {
            return <div class="border bg-danger/10 text-danger border-danger/20 rounded px-4 py-3 flex justify-between items-center">
                 revision metadata is not valid {result()?.file_verification.message}
            </div>
        }
    }

    let showRevisionContentVerificationResults = () => {
        if (result()?.metadata_verification.successful) {
            return <div class="border bg-success/10 text-info border-info/20 rounded px-4 py-3 flex justify-between items-center">
                 revision conents is valid
            </div>
        } else {
            return <div class="border bg-danger/10 text-danger border-danger/20 rounded px-4 py-3 flex justify-between items-center">
                 revision metadata is not valid {result()?.file_verification.message}
            </div>
        }
    }

    return (
        <div class="p-3 ms-5">
            <div class="flex items-center gap-3">
                <div class="h-10 w-10 flex-shrink-0">
                    {props.index + 1}
                </div>
                <div class="flex-grow truncate">

                    <i class="bi bi-check"></i>

                    {isLoading() ? <div /> : <div class="mt-5">{showRevisionSummary()} </div>}

                    {isLoading() ? <div /> : <div class="mt-5">{showRevisionFileVerificationResults()} </div>}

                    {isLoading() ? <div /> : <div class="mt-5">{showRevisionMetadataVerificationResults()} </div>}

                    {isLoading() ? <div /> : <div class="mt-5 mb-5">{showRevisionContentVerificationResults()} </div>}


                    <div  class="text-gray-600 dark:text-gray-400 mb-5 flex items-center" style="font-family : 'monospace' "> Verification Hash : {formatCryptoAddress(props.revisionHash, 20, 5)}  <div>{props.copyView(props.revisionHash,"Verification hash copied to clipboard!")}</div> </div>
                    <p class="text-gray-600 dark:text-gray-400 mt-5" style="font-family : 'monospace' "> Previous Verification Hash : {formatCryptoAddress(props.revision.metadata.previous_verification_hash ?? "", 20, 5)}  </p>
                    <p class="text-gray-600 dark:text-gray-400 mt-5" style="font-family : 'monospace' "> Content Hash : {formatCryptoAddress(props.revision.content.content_hash, 20, 5)}  </p>
                    <p class="text-gray-600 dark:text-gray-400 mt-5" style="font-family : 'monospace' "> Metadat Hash : {formatCryptoAddress(props.revision.metadata.metadata_hash, 20, 5)}  </p>
                    <p class="text-gray-600 dark:text-gray-400 mt-5" style="font-family : 'monospace' "> Time stamp : {timeToHumanFriendly(props.revision.metadata.time_stamp)}  </p>

                    <br />
                    {props.revision.signature == null ? <h2 style={{ "margin-bottom": "18px" }}>No signature</h2> : <div style={{ "margin-bottom": "18px" }} >
                        <h6 style={{ "margin-block": "20px" }}>Signature details</h6>

                        <DetailsPageSignature signature={props.revision.signature} previous_verification_hash={props.revision.metadata.previous_verification_hash ?? ""} />
                    </div>}
                    <br />
                    {props.revision.witness == null ? <h2 style={{ "margin-bottom": "18px" }}>No witness</h2> : <div style={{ "margin-bottom": "18px" }} >
                        <h6 style={{ "margin-block": "20px" }}>Witness details</h6>

                        <DetailsPageWitness witness={props.revision.witness} previous_verification_hash={props.revision.metadata.previous_verification_hash ?? ""} />
                    </div>}

                </div>

            </div>
            <hr />
            <br></br>
        </div>
    );
};
