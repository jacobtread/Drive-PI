import { FunctionComponent } from "react";
import Heading from "$components/Heading";
import Drives from "$components/Drives";

export const Home: FunctionComponent = () => {
    return (
        <div className="home">
            <Heading/>
            <main className="home__main">
                <Drives/>
                <div>

                </div>
            </main>
        </div>
    );
}

export default Home;