import React from 'react';
import { Route, Routes } from "react-router-dom";
import Home from "./routes/Home";
import Auth from "./routes/Auth";
import { AccessProvider, RequireAccess } from "./components/AccessProvider";


function App() {
    return (
        <AccessProvider>
            <Routes>
                <Route path={"/auth"} element={<Auth/>}></Route>
                <Route element={<RequireAccess/>}>
                    <Route path={"/"} element={<Home/>}></Route>
                </Route>
            </Routes>
        </AccessProvider>
    );
}

export default App;
