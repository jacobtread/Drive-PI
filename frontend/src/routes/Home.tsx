import { FunctionComponent } from "react";
import { useAccess } from "$components/AccessProvider";
import Heading from "$components/Heading";
import { Outlet } from "react-router-dom";

export const Home: FunctionComponent = () => {
    return (
        <div className="home">

            <Heading/>
            <main className="home__main">
                <h1>Home</h1>
                <Outlet/>
            </main>
        </div>
    );
}

export default Home;