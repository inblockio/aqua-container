import { Component, createEffect, createSignal, For, JSX, onCleanup } from "solid-js";
import { appState, setAppState } from "../store/store";
import { HashChain, PageData, Revision, RevisionSignature, RevisionWitness } from "../models/PageData";
import { useNavigate } from "@solidjs/router";
import { fileType, formatCryptoAddress, timeToHumanFriendly } from "../util";
import {AquaVerifier } from "aqua-verifier";

const DetailsPage: Component = () => {

    const navigate = useNavigate();
    const [filePageData, setFilePageData] = createSignal<PageData | undefined>();
    createEffect(() => {
        let sample = new AquaVerifier();
        
        if (appState.selectedFileFromApi != undefined) {
            const pageData: PageData = JSON.parse(appState.selectedFileFromApi.page_data);
            setFilePageData(pageData)
        } else {
            navigate("/details");
        }


        // This will run when the component is unmounted
        onCleanup(() => {
            console.log("DetailsPage Component unmounted");
            setAppState('selectedFileFromApi', undefined)

        });
    });

    createEffect(() => {

        if (appState.selectedFileFromApi != undefined) {
            const pageData: PageData = JSON.parse(appState.selectedFileFromApi.page_data);
            setFilePageData(pageData)
        } else {
            navigate("/details");
        }

    }, [appState.selectedFileFromApi]);

    const fileHashAndRevisionsDetails = () => {
        return <For each={filePageData()?.pages ?? []}>
            {(item, index) =>
                <>
                    {fileChainsDisplay(item, index())}
                </>
            }

        </For>
    }
    const fileChainsDisplay = (pages: HashChain, index: number) => {
        return <div class="p-3">
            <div class="flex items-center gap-3">
                <div class="h-10 w-10 flex-shrink-0">
                    {index + 1}
                </div>
                <div class="flex-grow truncate mb-5">
                    <div class="font-medium text-gray-900 dark:text-gray-300 truncate mb-3"> Domain :  {pages.domain_id}                    </div>
                    <div class="font-medium text-gray-900 dark:text-gray-300 flex items-center">
                        Genesis Hash: {formatCryptoAddress(pages.genesis_hash)}
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="16"
                            height="16"
                            fill="currentColor"
                            class="bi bi-copy ml-2"  // added margin-left to space it
                            viewBox="0 0 16 16"
                        >
                            <path fill-rule="evenodd" d="M4 2a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2zm2-1a1 1 0 0 0-1 1v8a1 1 0 0 0 1 1h8a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1zM2 5a1 1 0 0 0-1 1v8a1 1 0 0 0 1 1h8a1 1 0 0 0 1-1v-1h1v1a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h1v1z" />
                        </svg>
                    </div>
                    {/* <p class="text-gray-600 dark:text-gray-400"> </p> */}
                </div>


            </div>
            <div class="ms-5 mt-5 ps-5" style={{ "margin-left": "30px" }}>
                <hr />
                <br />
                <h6>Revisions</h6>
                <br />
                <For each={filePageData()?.pages ?? []}>
                    {(item, index) =>
                        <>
                            {fileRevisionsDisplay(index())}
                        </>
                    }

                </For>
            </div>
        </div>
    }


    const fileRevisionsDisplay = (indexPar: number) => {
        let totalRevisions = filePageData()?.pages[indexPar].revisions;
        if (totalRevisions != null) {

            console.log("revisions are not empty ");
            let index = 0;
            const revisionsJSXArray: JSX.Element[] = [];
            for (const [key, revision] of Object.entries(totalRevisions)) {
                console.log(`Revision Key: ${key}, Revision ID: ${revision.witness}, Content: ${revision.content}`);
                // You can add any processing logic you want here
                let rev = fileRevisionsDisplayItem(revision, index, key);
                revisionsJSXArray.push(rev);
                index++;
            }

            return <div>{revisionsJSXArray}</div>;

        } else {
            return <div>No revisions found</div>
        }
    }
    const fileRevisionsDisplayItem = (revision: Revision, index: number, revisionHash: string) => {
        return <div class="p-3 ms-5">
            <div class="flex items-center gap-3">
                <div class="h-10 w-10 flex-shrink-0">
                    {index + 1}
                </div>
                <div class="flex-grow truncate">

                    <p class="text-gray-600 dark:text-gray-400 mb-5" style="font-family : 'monospace' "> Verification Hash : {formatCryptoAddress(revisionHash, 20, 5)}  </p>
                    <p class="text-gray-600 dark:text-gray-400 mt-5" style="font-family : 'monospace' "> Previous Verification Hash : {formatCryptoAddress(revision.metadata.previous_verification_hash ?? "", 20, 5)}  </p>
                    <p class="text-gray-600 dark:text-gray-400 mt-5" style="font-family : 'monospace' "> Content Hash : {formatCryptoAddress(revision.content.content_hash, 20, 5)}  </p>
                    <p class="text-gray-600 dark:text-gray-400 mt-5" style="font-family : 'monospace' "> Metadat Hash : {formatCryptoAddress(revision.metadata.metadata_hash, 20, 5)}  </p>
                    <p class="text-gray-600 dark:text-gray-400 mt-5" style="font-family : 'monospace' "> Time stamp : {timeToHumanFriendly(revision.metadata.time_stamp)}  </p>

                    <br />
                    {revision.signature == null ? <h2 style={{"margin-bottom" : "18px"}}>No signature</h2> : <div style={{"margin-bottom" : "18px"}} >
                        <h6 style={{"margin-block": "20px"}}>Signature details</h6>
                        {revisionSignatureDisplay(revision.signature)}
                    </div>}
                    <br />
                    {revision.witness == null ? <h2 style={{"margin-bottom" : "18px"}}>No witness</h2> : <div style={{"margin-bottom" : "18px"}} >
                        <h6 style={{"margin-block": "20px"}}>Witness details</h6>
                        {revisionWitnessDisplay(revision.witness)}
                    </div>}

                </div>

            </div>
            <hr />
            <br></br>
        </div>

    }

    const revisionSignatureDisplay = (signature: RevisionSignature) => {
        return <div style={{ "margin-left": "30px" }}>
            <div class="flex-grow truncate">

                <p class="text-gray-600 dark:text-gray-400 mb-5" style="font-family : 'monospace' "> Signature : {formatCryptoAddress(signature.signature, 20, 5)}  </p>
                <p class="text-gray-600 dark:text-gray-400 mb-5" style="font-family : 'monospace' "> Signature Hash : {formatCryptoAddress(signature.signature_hash, 20, 5)}  </p>
                <p class="text-gray-600 dark:text-gray-400 mb-5" style="font-family : 'monospace' ">  wallet Address  : {formatCryptoAddress(signature.wallet_address, 20, 5)}  </p>
                <p class="text-gray-600 dark:text-gray-400 mb-5" style="font-family : 'monospace' ">  public key  : {formatCryptoAddress(signature.public_key, 20, 5)}  </p>
            </div>

        </div>
    }
    const revisionWitnessDisplay = (witnnes: RevisionWitness) => {
        return <div style={{ "margin-left": "30px" }}>
        <div class="flex-grow truncate">
            <p class="text-gray-600 dark:text-gray-400 mb-5" style="font-family : 'monospace' "> Witeness network : {witnnes.witness_network}  </p>
            <p class="text-gray-600 dark:text-gray-400 mb-5" style="font-family : 'monospace' "> Witness hash : {formatCryptoAddress(witnnes.witness_hash, 20, 5)}  </p>
            <p class="text-gray-600 dark:text-gray-400 mb-5" style="font-family : 'monospace' "> Witness domain snapshot genesis hash : {formatCryptoAddress(witnnes.domain_snapshot_genesis_hash, 20, 5)}  </p>
            <p class="text-gray-600 dark:text-gray-400 mb-5" style="font-family : 'monospace' "> Witness  event  transaction hash : {formatCryptoAddress(witnnes.witness_event_transaction_hash ,20, 5)}  </p>
            <p class="text-gray-600 dark:text-gray-400 mb-5" style="font-family : 'monospace' "> Witness  event  verifiction hash : {formatCryptoAddress(witnnes.witness_event_verification_hash, 20, 5)}  </p>
        </div>

    </div>
    }
    const filePreviewView = () => {

        if (appState.selectedFileFromApi == undefined) {
            return <div>

            </div>
        }
        const fileTypeInfo = fileType(appState.selectedFileFromApi);

        if (filePageData() && filePageData()?.pages != null && filePageData()?.pages.length > 0) {
            const firstPage = filePageData()!.pages[0]; // Get the first page
            const firstRevisionKey = Object.keys(firstPage.revisions)[0]; // Get the first revision key
            const firstRevision = firstPage.revisions[firstRevisionKey]; // Get the first revision
            const fileContent = firstRevision.content.file; // Get file content

            if (fileContent && fileTypeInfo === "Image") {
                const base64String = `data:image/png;base64,${fileContent.data}`;
                return <img id="base64Image" alt="Base64 Image" src={base64String}></img>
            }
        }
        return <div class="text-center m-5">
            <img id="base64Image" alt="Base64 Image" class="rounded  img-fluid" src="/images/preview.jpg"></img>
        </div>
    }
    const fileRevisionsText = () => {
        let totalRevisions = filePageData()?.pages[0].revisions;
        let fileRevisionsCount = 0;
        if (totalRevisions != null) {
            fileRevisionsCount = Object.keys(totalRevisions).length;
        }

        return fileRevisionsCount;

    }

    const fileSignersText = () => {
        let signersCount = 0;
        let totalRevisions = filePageData()?.pages[0].revisions;
        if (totalRevisions != null) {

            console.log("revisions are not empty ")

            for (const [key, revision] of Object.entries(totalRevisions)) {
                console.log(`Revision Key: ${key}, Revision ID: ${revision.witness}, Content: ${revision.content}`);
                // You can add any processing logic you want here
                if (revision.signature != null) {
                    signersCount += 1
                }
            }
        }
        return signersCount;


    }


    const fileWitnessText = () => {
        let witnessCount = 0;
        let totalRevisions = filePageData()?.pages[0].revisions;
        if (totalRevisions != null) {

            console.log("revisions are not empty ")

            for (const [key, revision] of Object.entries(totalRevisions)) {
                console.log(`Revision Key: ${key}, Revision ID: ${revision.witness}, Content: ${revision.content}`);
                // You can add any processing logic you want here
                if (revision.witness != null) {
                    witnessCount += 1
                }
            }
        }
        return witnessCount;


    }


    return (
        <>
            <div class="flex wrapper">



                {/*   <!--==============================================================--> */}
                {/*   <!--Start Page Content here--> */}
                {/*   <!--==============================================================--> */}

                <div class="page-content">


                    <main class="flex-grow p-6">

                        {/*   <!--Page Title Start--> */}
                        <div class="flex justify-between items-center mb-6">
                            <h4 class="text-slate-900 dark:text-slate-200 text-lg font-medium">File Detail</h4>


                        </div>
                        {/*   <!--Page Title End--> */}

                        <div class="grid lg:grid-cols-3 gap-6">
                            <div class="col-span-2">
                                <div class="card">
                                    <div class="card-header">
                                        <h6 class="card-title">File Overview</h6>
                                    </div>

                                    <div class="p-6">
                                        <div class="grid lg:grid-cols-4 gap-6">
                                            {/* <!-- stat 1 --> */}
                                            <div class="flex items-center gap-5">
                                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-grid h-10 w-10"><rect x="3" y="3" width="7" height="7"></rect><rect x="14" y="3" width="7" height="7"></rect><rect x="14" y="14" width="7" height="7"></rect><rect x="3" y="14" width="7" height="7"></rect></svg>

                                                <div class="">
                                                    <h4 class="text-lg text-gray-700 dark:text-gray-300 font-medium">{fileRevisionsText()}</h4>
                                                    <span class="text-sm">Total Revisions</span>
                                                </div>
                                            </div>

                                            {/* <!-- stat 2 --> */}
                                            <div class="flex items-center gap-5">
                                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-check-square h-10 w-10"><polyline points="9 11 12 14 22 4"></polyline><path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11"></path></svg>
                                                <div class="">
                                                    <h4 class="text-lg text-gray-700 dark:text-gray-300 font-medium">{fileSignersText()}</h4>
                                                    <span class="text-sm">Total Signers</span>
                                                </div>
                                            </div>

                                            {/* <!-- stat 3 --> */}
                                            <div class="flex items-center gap-5">
                                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-users h-10 w-10"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path><circle cx="9" cy="7" r="4"></circle><path d="M23 21v-2a4 4 0 0 0-3-3.87"></path><path d="M16 3.13a4 4 0 0 1 0 7.75"></path></svg>
                                                <div class="">
                                                    <h4 class="text-lg text-gray-700 dark:text-gray-300 font-medium">{fileWitnessText()}</h4>
                                                    <span class="text-sm">Total Witness</span>
                                                </div>
                                            </div>
                                            {/* <!-- stat 3 --> */}
                                            {/* <div class="flex items-center gap-5">
                                                <i data-feather="clock" class="h-10 w-10"></i>
                                                <div class="">
                                                    <h4 class="text-lg text-gray-700 dark:text-gray-300 font-medium">2500</h4>
                                                    <span class="text-sm">Total Hours Spent</span>
                                                </div>
                                            </div> */}
                                        </div>
                                    </div>
                                </div>
                                <br />
                                <div class="card">
                                    <div class="card-header">
                                        <h6 class="card-title">File Revisions</h6>
                                    </div>

                                    <div class="table overflow-hidden w-full">
                                        <div
                                            class="divide-y divide-gray-300 dark:divide-gray-700 overflow-auto w-full max-w-full">

                                            {

                                                fileHashAndRevisionsDetails()
                                            }
                                        </div>
                                    </div>
                                </div>
                            </div>



                            <div class="col-span-1">
                                <div class="card">
                                    <div class="card-header">
                                        <h6 class="card-title">File Preview</h6>
                                    </div>
                                    <div class="table overflow-hidden w-full">
                                        <div class="divide-y divide-gray-300 dark:divide-gray-700 overflow-auto w-full max-w-full">
                                            {filePreviewView()}
                                        </div>
                                    </div>
                                </div>
                            </div>


                        </div>
                    </main>

                    {/*   <!--Footer Start--> */}
                    <footer class="footer h-16 flex items-center px-6 bg-white shadow dark:bg-gray-800">
                        <div class="flex justify-center w-full gap-4">
                            <div>
                                <script>document.write(new Date().getFullYear())</script>
                                ©  - <a href="https://github.com/inblockio" target="_blank">Inblock</a>
                            </div>
                        </div>
                    </footer>
                    {/*   <!--Footer End--> */}

                </div>

                {/*   <!--==============================================================--> */}
                {/*   <!--End Page content--> */}
                {/*   <!--==============================================================--> */}

            </div>
        </>
    )
}


export default DetailsPage;