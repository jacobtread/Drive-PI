import React, { FunctionComponent } from "react";
import Home from "$routes/Home";
import Auth from "$routes/Auth";
import { useHasAccess } from "$components/AccessProvider";

/**
 * Root app components which contains all the routes
 * and the access provider
 */
const App: FunctionComponent = () => {
    const hasAccess = useHasAccess()
    if (hasAccess) {
        return <Home/>
    } else {
        return <Auth/>
    }
}

export default App;