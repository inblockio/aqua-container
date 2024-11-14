
import { openDB } from 'idb';
import { createStore } from 'zustand';
import { createJSONStorage, persist } from 'zustand/middleware';
import { ApiFileInfo } from './models/FileInfo';

type AppStoreState = {
    configuration: {
        network: string,
        domain: string,
        fileMode: string,
        contractAddress: string | null
    },
    files: ApiFileInfo[],
    metamaskAddress: string | null
}

type AppStoreActions = {
    setConfiguration: (
        config: AppStoreState['configuration'],
    ) => void,
    setMetamaskAddress: (
        address: AppStoreState['metamaskAddress'],
    ) => void,
    setFiles: (
        files: AppStoreState['files'],
    ) => void

}

type TAppStore = AppStoreState & AppStoreActions


// Open an IndexedDB instance
const dbPromise = openDB('aquafier-db', 1, {
    upgrade(db) {
      db.createObjectStore('store');
    },
  });
  
  // Custom storage object for Zustand using IndexedDB
  const indexedDBStorage = {
    getItem: async (name: string) => {
      const db = await dbPromise;
      return (await db.get('store', name)) || null;
    },
    setItem: async (name: string, value: string) => {
      const db = await dbPromise;
      await db.put('store', value, name);
    },
    removeItem: async (name: string) => {
      const db = await dbPromise;
      await db.delete('store', name);
    },
  };

const appStore = createStore<TAppStore>()(
    persist(
        (set) => ({
            // Initial state
            configuration: {
                network: 'mainnet',
                domain: 'dHy4ds_dsf',
                fileMode: 'public',
                contractAddress: '',
            },
            files: [],
            metamaskAddress: '',
            // Actions
            setConfiguration: (config) => set({ configuration: config }),
            setMetamaskAddress: (
                address: AppStoreState['metamaskAddress'],
            ) => set({ metamaskAddress: address }),
            setFiles: (
                files: AppStoreState['files'],
            ) => set({ files: files }),
        }),
        {
            name: 'app-store', // Unique name for storage key
            storage: createJSONStorage(() => indexedDBStorage)
        }
    )
);

export default appStore;
