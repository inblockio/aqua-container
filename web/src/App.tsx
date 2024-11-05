import type { Component } from 'solid-js';

import HomePage from "./pages/home";
import DetailsPage from "./pages/details"
import { Route, Router } from "@solidjs/router";
import ConfigsPage from './pages/configuration';
import MainLayout from './layout/MainLayout';
import { ethers } from 'ethers';
import LoadConfiguration from './pages/LoadConfiguration';


declare global {
  interface Window {
    ethereum?: ethers.Eip1193Provider;
  }
}

const App: Component = () => {

  return (
    <MainLayout>
      <LoadConfiguration />
      <Router>
        <Route path="/configuration" component={ConfigsPage} />
        <Route path="/details" component={DetailsPage} />
        <Route path="/" component={HomePage} />
        <Route path="*" component={HomePage} />
      </Router>
    </MainLayout>
  );
};

export default App;
