import { Component, JSX } from "solid-js"
import CustomNavbar from "../components/CustomNavbar"


const MainLayout: Component<{children: JSX.Element }> = ({ children }) => {

    return (
        <div>
            <CustomNavbar />
            {children}
        </div>
    )
}

export default MainLayout