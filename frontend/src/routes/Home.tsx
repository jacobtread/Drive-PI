import { FunctionComponent } from "react";
import { useAccess } from "../components/AccessProvider";

const Home: FunctionComponent = () => {
    const {logout} = useAccess()
    return (
        <div>
            <h1>Home</h1>
            <button onClick={logout}>
                Logout
            </button>
        </div>
    )
}

export default Home