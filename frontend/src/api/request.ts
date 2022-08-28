import { Token } from "../components/AccessProvider";

const API_BASE_URL: string = process.env.REACT_APP_URL ?? "/api";

export type RequestMethod = "GET" | "POST" | "PUT" | "DELETE";

export interface RequestData {
    method: RequestMethod,
    path: string;
    body?: any;
}

export async function request<T>(requestData: RequestData, token: Token = null): Promise<T> {
    const init: RequestInit = {method: requestData.method}
    const headers: Record<string, string> = {}
    if (token != null) {
        headers["X-Token"] = token
    }
    if (requestData.method !== "GET" && requestData.body !== null) {
        headers["Content-Type"] = "application/json"
        init.body = JSON.stringify(requestData.body)
    }
    init.headers = headers
    const response = await fetch(`${API_BASE_URL}/${requestData.path}`, init)
    return await response.json()
}