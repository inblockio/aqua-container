import { Component, createEffect, createSignal, For, onCleanup } from "solid-js";
import { appState, setAppState } from "../store/store";
import { HashChain, PageData, Revision } from "../models/PageData";
import { FileInfo } from "../models/FileInfo";
import { useNavigate } from "@solidjs/router";
import { fileType } from "../util";


const DetailsPage: Component = () => {

    const navigate = useNavigate();
    const [filePageData, setFilePageData] = createSignal<PageData | undefined>();
    createEffect(() => {
        if (appState.selectedFileFromApi != undefined) {
            const pageData: PageData = JSON.parse(appState.selectedFileFromApi.page_data);
            setFilePageData(pageData)
        } else {
            // navigate("/details");
        }


        // This will run when the component is unmounted
        onCleanup(() => {
            console.log("DetailsPage Component unmounted");
            setAppState('selectedFileFromApi', undefined)

        });
    });


    const fileHashAndRevisionsDetails = () => {
      return  <For each={filePageData()?.pages ?? []}>
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
                <div class="flex-grow truncate">
                    <div class="font-medium text-gray-900 dark:text-gray-300 truncate"> Domain :  {pages.domain_id}

                    </div>
                    <p class="text-gray-600 dark:text-gray-400"> Hash : </p>
                </div>

                {/* <For each={filePageData()?.pages ?? []}>
                    {(item, index) =>
                        <>
                            {fileRevisionsDisplay(item, index())}
                        </>
                    }

                </For> */}

            </div>
        </div>
    }
    const fileRevisionsDisplay = (pages: Revision, index: number) => {
        return <div class="p-3">
            <div class="flex items-center gap-3">
                <div class="h-10 w-10 flex-shrink-0">
                    {index + 1}
                </div>
                <div class="flex-grow truncate">
                    <div class="font-medium text-gray-900 dark:text-gray-300 truncate"> Domain :  {pages.domain_id}

                    </div>
                    <p class="text-gray-600 dark:text-gray-400"> Hash : </p>
                </div>
                {/* <div
                        class="px-3 py-1 md:block hidden rounded text-xs font-medium">Testing
                    </div>
                    <div class="ms-auto">
                        <div
                            class=" px-3 py-1 rounded text-xs font-medium bg-success/25 text-success">Complated
                        </div>
                    </div> */}
            </div>
        </div>

    }
    const filePreviewView = () => {

        const fileTypeInfo = fileType(appState.selectedFileFromApi!!);

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
        if (totalRevisions != null) {

            let length = Object.keys(totalRevisions).length;

            return length;
        } else {
            console.log("revisions are null ...")
            return 0;
        }

    }

    // const fileRevisionsDisplay = (revision: Revision) => {
    //     return <>
    //     </>
    // }
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
                                                    <h4 class="text-lg text-gray-700 dark:text-gray-300 font-medium">0</h4>
                                                    <span class="text-sm">Total Signers</span>
                                                </div>
                                            </div>

                                            {/* <!-- stat 3 --> */}
                                            <div class="flex items-center gap-5">
                                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-users h-10 w-10"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path><circle cx="9" cy="7" r="4"></circle><path d="M23 21v-2a4 4 0 0 0-3-3.87"></path><path d="M16 3.13a4 4 0 0 1 0 7.75"></path></svg>
                                                <div class="">
                                                    <h4 class="text-lg text-gray-700 dark:text-gray-300 font-medium">0</h4>
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