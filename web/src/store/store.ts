import {FileInfo} from "../models/FileInfo";
import {createStore} from "solid-js/store";

export interface AppState {
    filesFromApi: Array<FileInfo>;
    selectedFileFromApi : FileInfo |  undefined,
    metaMaskAddress: string | null
}


const initialState : AppState = {
    filesFromApi: [],
    selectedFileFromApi: undefined,
    metaMaskAddress: null
}

export const [appState, setAppState] = createStore(initialState);
