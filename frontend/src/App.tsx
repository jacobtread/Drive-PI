import React, { FunctionComponent, useState } from "react";
import Auth from "$components/Auth";
import { useHasAccess } from "$components/AccessProvider";
import Heading from "$components/Heading";
import Drives, { Drive } from "$components/Drives";
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

    const [selected, setSelected] = useState<Drive | null>(null)

    return (
        <div className="home">
            <Heading/>
            <main className="main">
                <Drives selected={selected} setSelected={setSelected}/>
                <FileBrowser drive={selected}/>
            </main>
        </div>
    )
}

export default App;