import { ethers } from 'ethers';
import MainLayout from './layouts/MainLayout'
import Home from './pages/Home'
import LoadConfiguration from './components/config';
import { initializeBackendUrl } from './utils/constants';
import { useEffect } from 'react'
import appStore from './store';
import { useStore } from "zustand"
import { BrowserRouter, Route, Routes } from 'react-router-dom'
import SharePage from './pages/SharePage';
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
    <BrowserRouter>
      <LoadConfiguration />
      <MainLayout>
        <Routes>
          <Route path="" element={<Home />} />
          <Route path="/share/:identifier" element={<SharePage />} />
        </Routes>
      </MainLayout>
    </BrowserRouter>
  )
}

export default App