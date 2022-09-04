import React from "react";
import ReactDOM from "react-dom/client";
import App from "$app/App";
import "$assets/styles/styles.scss";
import { AccessProvider } from "$components/AccessProvider";

// Create react dom root element at the html #root element
const root = ReactDOM.createRoot(document.getElementById("root") as HTMLElement);
// Render the app wrapped in the access provider
root.render(
    <React.StrictMode>
        <AccessProvider>
            <App/>
        </AccessProvider>
    </React.StrictMode>
);
