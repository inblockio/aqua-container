import { ethers } from 'ethers';
import MainLayout from './layouts/MainLayout'
import Home from './pages/Home'
import LoadConfiguration from './components/config';

declare global {
  interface Window {
    ethereum?: ethers.Eip1193Provider;
  }
}


function App() {

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
