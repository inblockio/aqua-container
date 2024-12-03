import { ethers } from 'ethers';
import MainLayout from './layouts/MainLayout'
import Home from './pages/Home'
import LoadConfiguration from './components/config';
import { useEffect } from "react";

declare global {
  interface Window {
    ethereum?: ethers.Eip1193Provider;
  }
}


function App() {

  useEffect(() => {
    console.log("HOST API ="+import.meta.env.VITE_REMOTE);
    console.log("HOST PORT ="+import.meta.env.VITE_REMOTE_PORT)
  });

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
