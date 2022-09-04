import React, { FunctionComponent, useState } from "react";
import Auth from "$components/Auth";
import { useAccess } from "$components/AccessProvider";
import Drives from "$components/Drives";
import FileBrowser from "$components/FileBrowser";
import { DriveItem } from "$api/models";
import LogoSide from "$assets/images/logo-side.svg"

/**
 * Root app components which contains all the routes
 * and the access provider
 */
const App: FunctionComponent = () => {
    const {token, logout} = useAccess();
    const [selected, setSelected] = useState<DriveItem | null>(null)

    if (token === null) {
        return <Auth/>
    }

    return (
        <div className="home">
            <div className="heading">
                <img src={LogoSide} alt="Logo" className="heading__logo"/>
                <div>
                    <h1 className="heading__title">Drive-PI</h1>
                    <button onClick={logout} className="button heading__logout">
                        Logout
                    </button>
                </div>
            </div>
            <main className="main">
                <Drives selected={selected} setSelected={setSelected}/>
                <FileBrowser drive={selected}/>
            </main>
        </div>
    )
}

export default App;