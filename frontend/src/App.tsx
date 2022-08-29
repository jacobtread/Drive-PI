import React, { FunctionComponent } from "react";
import { Route, Routes } from "react-router-dom";
import Home from "$routes/Home";
import Auth from "$routes/Auth";
import { AccessProvider, RequireAccess } from "$components/AccessProvider";

/**
 * Root app components which contains all the routes
 * and the access provider
 */
const App: FunctionComponent = () => (
    // Access Provider provides the access context
    <AccessProvider>
        <Routes>
            <Route path="/auth" element={<Auth/>}></Route>
            {/* Routes that require authentication wrapped with <RequireAccess/> */}
            <Route element={<RequireAccess/>}>
                <Route path="/" element={<Home/>}></Route>
            </Route>
        </Routes>
    </AccessProvider>
);

export default App;