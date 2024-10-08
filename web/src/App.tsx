import type {Component} from 'solid-js';

import HomePage from "./pages/home";
import DetailsPage from "./pages/details"
import {Route} from "@solidjs/router";
const App: Component = () => {


    return (
     <>

         <Route path="/details" component={DetailsPage}/>
         <Route path="/" component={HomePage}/>
         <Route path="*" component={HomePage}/>

     </>
    );
};

export default App;
