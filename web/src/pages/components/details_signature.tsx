import { Component, createEffect, createSignal } from "solid-js";
import { RevisionSignature } from "../../models/PageData";
import { formatCryptoAddress } from "../../util";
import AquaVerifier , {  ResultStatus } from "aqua-verifier";

interface DetailsPageSignatureProps {
    signature: RevisionSignature;
    previous_verification_hash: string
}

export const DetailsPageSignature: Component<DetailsPageSignatureProps> = (props) => {
    const [isLoading, setIsLoading] = createSignal(false);
    const [result, setResult] = createSignal<ResultStatus | null>(null);


    createEffect(() => {
        setIsLoading(true);
        let verifier = new AquaVerifier();
        let res = verifier.verifySignature(props.signature, props.previous_verification_hash)
        setResult(res);
        setIsLoading(false);
    });

    const isSignatureValid = () => {
        if (result == null) {
            return <div class="border bg-danger/10 text-info border-danger/20 rounded px-4 py-3 flex justify-between items-center mb-5 " role="alert">Signature is invalid, failed while validating </div>
        } else {
            if (result()?.successful) {
                return <div class="border bg-success/10 text-success border-success/20 rounded px-4 py-3 flex justify-between items-center mb-5" role="alert">Signature  is valid</div>
            } else {
                return <div class="border bg-danger/10 text-danger border-info/20 rounded px-4 py-3 flex justify-between items-center mb-5" role="alert"> Signature is invalid : {result()?.message} </div>
            }
        }
    }

    return (
        <div style={{ "margin-left": "30px" }}>

            {
                isLoading() ? <div class="border bg-info/10 text-info border-info/20 rounded px-4 py-3 flex justify-between items-center" role="alert">
                    Checking is signature is valid
                </div> : <div> {isSignatureValid()}</div>


            }

            <div class="flex-grow truncate">
                <p class="text-gray-600 dark:text-gray-400 mb-5" style="font-family: 'monospace'">
                    Signature: {formatCryptoAddress(props.signature.signature, 20, 5)}
                </p>
                <p class="text-gray-600 dark:text-gray-400 mb-5" style="font-family: 'monospace'">
                    Signature Hash: {formatCryptoAddress(props.signature.signature_hash, 20, 5)}
                </p>
                <p class="text-gray-600 dark:text-gray-400 mb-5" style="font-family: 'monospace'">
                    Wallet Address: {formatCryptoAddress(props.signature.wallet_address, 20, 5)}
                </p>
                <p class="text-gray-600 dark:text-gray-400 mb-5" style="font-family: 'monospace'">
                    Public Key: {formatCryptoAddress(props.signature.public_key, 20, 5)}
                </p>
            </div>
        </div>
    );
};
