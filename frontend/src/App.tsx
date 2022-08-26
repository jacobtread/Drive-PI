import React from 'react';
import './assets/styles/App.scss';
import { Link, Route, Routes } from "react-router-dom";
import Home from "./routes/Home";
import Auth from "./routes/Auth";

function App() {
    return (
        <div className="App">
            <header>
                <nav>
                    <Link to="/">Home</Link>
                    <Link to="/auth">Auth</Link>
                </nav>
            </header>
            <Routes>
                <Route path={"/"} element={<Home/>}></Route>
                <Route path={"/auth"} element={<Auth/>}></Route>
            </Routes>
        </div>
    );
}

export default App;
