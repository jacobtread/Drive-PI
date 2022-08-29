// Base url for accessing the API (e.g. http://localhost:8080)
const API_BASE_URL: string = import.meta.env.VITE_APP_URL;

export type Token = string | null;

export type RequestMethod = "GET" | "POST" | "PUT" | "DELETE";

export interface RequestData {
    method: RequestMethod;
    path: string;
    body?: any;
}

export function request<T>(requestData: RequestData, token: Token = null): Promise<T> {
    const init: RequestInit = {method: requestData.method};
    const headers: Record<string, string> = {};
    if (token != null) {
        headers["X-Token"] = token;
    }
    if (requestData.method !== "GET" && requestData.body) {
        headers["Content-Type"] = "application/json";
        init.body = JSON.stringify(requestData.body);
    }
    init.headers = headers;

    return new Promise((resolve, reject) => {
        fetch(`${API_BASE_URL}/${requestData.path}`, init)
            .then(response => {
                if (Math.floor(response.status / 100) === 2) {
                    response.json()
                        .then(resolve)
                        .catch(_ => reject([response.status, "Invalid JSON response"]));
                } else {
                    response.text()
                        .then(text => reject([response.status, text]))
                        .catch(_ => reject([response.status, "Unknown error"]));
                }
            })
            .catch(_ => reject([-1, "Failed to connect"]));
    })
}