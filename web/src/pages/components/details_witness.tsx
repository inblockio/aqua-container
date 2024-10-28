
import { Component, createEffect, createSignal } from "solid-js";
import { RevisionSignature, RevisionWitness } from "../../models/PageData";
import { formatCryptoAddress } from "../../util";
import { AquaVerifier, ResultStatus } from "aqua-verifier";

interface DetailsPageWitnessProps {
    witness: RevisionWitness;
    previous_verification_hash: string
}

export const DetailsPageWitness: Component<DetailsPageWitnessProps> = (props) => {
    const [isLoading, setIsLoading] = createSignal<boolean>(false);
    const [result, setResult] = createSignal<ResultStatus | null>(null);

    createEffect(async () => {
        setIsLoading(true);
        let verifier = new AquaVerifier();
        let item = props.witness;
        item.witness_event_transaction_hash = `0x${props.witness.witness_event_transaction_hash}`
        console.log("verify 0x in ", item);
        let res = await verifier.verifyWitness(item, props.previous_verification_hash, false);
        setResult(res);
        setIsLoading(false);
    });

    const isWitnessValid = () => {
        if (result == null) {
            return <div class="border bg-danger/10 text-danger border-danger/20 rounded px-4 py-3 flex justify-between items-center mb-5">Witness is invalid, failed while validating </div>
        } else {
            if (result()?.successful) {
                return <div class="border bg-success/10 text-success border-success/20 rounded px-4 py-3 flex justify-between items-center mb-5">Witness  is valid</div>
            } else {
                return <div  class="border bg-danger/10 text-danger border-danger/20 rounded px-4 py-3 flex justify-between items-center mb-5">
                Witness is invalid : {result()?.message} </div>
            }
        }
    }

    return (<div style={{ "margin-left": "30px" }}>
        <div class="flex-grow truncate">
            {
                isLoading() ? <div class="border bg-info/10 text-info border-info/20 rounded px-4 py-3 flex justify-between items-center">
                    Checking is witness is valid
                </div> : <div> {isWitnessValid()}</div>


            }
            <p class="text-gray-600 dark:text-gray-400 mb-5" style="font-family : 'monospace' "> Witeness network : {props.witness.witness_network}  </p>
            <p class="text-gray-600 dark:text-gray-400 mb-5" style="font-family : 'monospace' "> Witness hash : {formatCryptoAddress(props.witness.witness_hash, 20, 5)}  </p>
            <p class="text-gray-600 dark:text-gray-400 mb-5" style="font-family : 'monospace' "> Witness domain snapshot genesis hash : {formatCryptoAddress(props.witness.domain_snapshot_genesis_hash, 20, 5)}  </p>
            <p class="text-gray-600 dark:text-gray-400 mb-5" style="font-family : 'monospace' "> Witness  event  transaction hash : {formatCryptoAddress(props.witness.witness_event_transaction_hash, 20, 5)}  </p>
            <p class="text-gray-600 dark:text-gray-400 mb-5" style="font-family : 'monospace' "> Witness  event  verifiction hash : {formatCryptoAddress(props.witness.witness_event_verification_hash, 20, 5)}  </p>
        </div>

    </div>)
}