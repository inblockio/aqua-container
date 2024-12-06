import { ethers } from 'ethers';
import MainLayout from './layouts/MainLayout'
import Home from './pages/Home'
import LoadConfiguration from './components/config';
import { initializeBackendUrl } from './utils/constants';
import { useEffect } from 'react'
import appStore from './store';
import { useStore } from "zustand"
declare global {
  interface Window {
    ethereum?: ethers.Eip1193Provider;
  }
}

function App() {
  const { backend_url, setBackEndUrl } = useStore(appStore)

  useEffect(() => {
    console.log("backedn url is", backend_url);
    // Properly handle async initialization
    const initBackend = async () => {
     const url = await initializeBackendUrl();
     setBackEndUrl(url);
    };

    initBackend();
  }, []); // Empty dependency array means this runs once on mount

  return (
    <>
      <LoadConfiguration />
      <MainLayout>
        <Home />
      </MainLayout>
    </>
  )
}

export default App