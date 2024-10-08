import {FileInfo} from "../models/FileInfo";
import {createStore} from "solid-js/store";

export interface AppState {
    filesFromApi: Array<FileInfo>;
    selectedFileFromApi : FileInfo |  undefined
}


const initialState : AppState = {
    filesFromApi: [],
    selectedFileFromApi: undefined

}

export const [appState, setAppState] = createStore(initialState);
