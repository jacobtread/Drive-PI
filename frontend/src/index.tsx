import React from "react";
import ReactDOM from "react-dom/client";
import App from "$app/App";
import "$assets/styles/styles.scss";
import { AccessProvider } from "$components/AccessProvider";

const root = ReactDOM.createRoot(document.getElementById('root') as HTMLElement);
root.render(
    <React.StrictMode>
        <AccessProvider>
            <App/>
        </AccessProvider>
    </React.StrictMode>
);
