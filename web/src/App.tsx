import type {Component} from 'solid-js';

import HomePage from "./pages/home";
import DetailsPage from "./pages/details"
import {Route} from "@solidjs/router";
import ConfigsPage from './pages/configuration';

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
     <>
         <Route path="/configuration" component={ConfigsPage}/>
         <Route path="/details" component={DetailsPage}/>
         <Route path="/" component={HomePage}/>
         <Route path="*" component={HomePage}/>
     </>
    );
};

export default App;
