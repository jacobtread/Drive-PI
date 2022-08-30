import React, { FunctionComponent } from "react";
import Auth from "$components/Auth";
import { useHasAccess } from "$components/AccessProvider";
import Heading from "$components/Heading";
import Drives from "$components/Drives";
import FileBrowser from "$components/FileBrowser";

/**
 * Root app components which contains all the routes
 * and the access provider
 */
const App: FunctionComponent = () => {
    const hasAccess = useHasAccess()
    if (!hasAccess) {
        return <Auth/>
    }

    return (
        <div className="home">
            <Heading/>
            <main className="main">
                <Drives/>
                <FileBrowser/>
            </main>
        </div>
    )
}

export default App;