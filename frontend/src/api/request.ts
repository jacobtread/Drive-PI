// Base url for accessing the API (e.g. http://localhost:8080)
const API_BASE_URL: string = import.meta.env.VITE_APP_URL;

export type Token = string | null;

// Possible methods for the backend API
export type RouteMethod = "GET"
    | "POST"
    | "PUT"
    | "DELETE";

// Possible routes for the backend API
export type Route = "drives"
    | "files"
    | "auth"

export type ErrorResponse = [number, string];

export function request<T>(
    method: RouteMethod,
    route: Route,
    body: any = null,
    token: Token = null
): Promise<T> {
    const init: RequestInit = {method};
    const headers: Record<string, string> = {};
    if (token != null) {
        headers["X-Token"] = token;
    }
    if (method !== "GET" && body !== null) {
        headers["Content-Type"] = "application/json";
        init.body = JSON.stringify(body);
    }
    init.headers = headers;

    return new Promise((resolve, reject) => {
        fetch(`${API_BASE_URL}/${route}`, init)
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