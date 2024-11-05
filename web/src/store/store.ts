import {ApiFileInfo} from "../models/FileInfo";
import {createStore} from "solid-js/store";

interface Config {
    domain: string
    contractAddress: string
    chain: string
    fileMode: string
}

export interface AppState {
    filesFromApi: Array<ApiFileInfo>;
    selectedFileFromApi : ApiFileInfo |  undefined,
    metaMaskAddress: string | null,
    config: Config
}


const initialState : AppState = {
    filesFromApi: [],
    selectedFileFromApi: undefined,
    metaMaskAddress: null,
    config: {
        domain: '',
        contractAddress: '',
        chain: '',
        fileMode: ''
    }
}

export const [appState, setAppState] = createStore(initialState);
