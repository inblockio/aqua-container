import type { Component } from 'solid-js';

import HomePage from "./pages/home";
import DetailsPage from "./pages/details"
import { Route, Router } from "@solidjs/router";
import ConfigsPage from './pages/configuration';
import MainLayout from './layout/MainLayout';

declare global {
  interface Window {
    ethereum?: {
      isMetaMask?: boolean;
      request?: (args: { method: string; params?: Array<any> }) => Promise<any>;
    };
  }
}

const App: Component = () => {

  return (
    <MainLayout>
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
