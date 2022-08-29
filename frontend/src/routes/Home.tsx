import { FunctionComponent } from "react";
import { useAccess } from "../components/AccessProvider";
import Sidebar from "../components/Sidebar";

const Home: FunctionComponent = () => {
    const {logout} = useAccess()
    return (
        <div>
            <Sidebar/>
            <main></main>

            <h1>Home</h1>
            <button onClick={logout}>
                Logout
            </button>
        </div>
    )
}

export default Home