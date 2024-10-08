import {Component, createEffect, createSignal, For, onCleanup} from "solid-js";
import {appState, setAppState} from "../store/store";
import {HashChain, PageData, Revision} from "../models/PageData";
import {FileInfo} from "../models/FileInfo";


const DetailsPage: Component = () => {


    const [filePageData, setFilePageData] = createSignal<PageData | undefined>();
    createEffect(()=>{
        if(appState.selectedFileFromApi != undefined) {
            const pageData: PageData = JSON.parse(appState.selectedFileFromApi.page_data);
            setFilePageData(pageData)
        }


        // This will run when the component is unmounted
        onCleanup(() => {
            console.log("DetailsPage Component unmounted");
            setAppState('selectedFileFromApi', undefined)

        });
    });


    const fileChainsDisplay = (pages: HashChain, index : number)=>{
        return <div>
            <div class="p-3">
                <div class="flex items-center gap-3">
                    <div class="h-10 w-10 flex-shrink-0">
                        {index+1}
                    </div>
                    <div class="flex-grow truncate">
                        <div class="font-medium text-gray-900 dark:text-gray-300">Genesis Hash : {pages.genesis_hash}
                            Obama
                        </div>
                        <p class="text-gray-600 dark:text-gray-400">barakobama@gmail.com</p>
                    </div>
                    <div
                        class="px-3 py-1 md:block hidden rounded text-xs font-medium">Testing
                    </div>
                    <div class="ms-auto">
                        <div
                            class=" px-3 py-1 rounded text-xs font-medium bg-success/25 text-success">Complated
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
    const fileRevisionsDisplay = (revision : Revision)=>{
        return <>
        </>
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
                            <div class="lg:col-span-3">
                                <div class="card">
                                    <div class="card-header">
                                        <h6 class="card-title">File Overview</h6>
                                    </div>

                                    <div class="p-6">
                                        <div class="grid lg:grid-cols-4 gap-6">
                                            {/*   <!--stat 1--> */}
                                            <div class="flex items-center gap-5">
                                                <i data-feather="grid" class="h-10 w-10"></i>
                                                <div class="">
                                                    <h4 class="text-lg text-gray-700 dark:text-gray-300 font-medium">{filePageData()?.pages.length+1}</h4>
                                                    <span class="text-sm">Total Hash chains</span>
                                                </div>
                                            </div>


                                        </div>
                                    </div>
                                </div>
                            </div>



                            <div class="col-span-3">
                                <div class="card">
                                    <div class="card-header">
                                        <h6 class="card-title">File Revisions</h6>
                                    </div>

                                    <div class="table overflow-hidden w-full">
                                        <div
                                            class="divide-y divide-gray-300 dark:divide-gray-700 overflow-auto w-full max-w-full">

                                            {

                                                <For each={filePageData()?.pages ?? []}>
                                                    {(item, index) =>
                                                        <>
                                                            {fileChainsDisplay(item, index())}
                                                        </>
                                                    }

                                                </For>
                                            }
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
                                © Konrix - <a href="https://coderthemes.com/" target="_blank">Coderthemes</a>
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