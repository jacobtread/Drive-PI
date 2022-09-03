import { createContext, FunctionComponent, PropsWithChildren, useContext, useEffect, useState } from "react";
import { request, Route, RouteMethod, Token } from "$api/request";

const LOCAL_STORAGE_KEY: string = "drivepi_token";

interface AccessContextType {
    token: Token;
    setToken: (token: Token) => void;
    request: <V>(method: RouteMethod, path: Route, body?: any) => Promise<V>;
    logout: () => void;
}

interface CheckResponse {
    valid: boolean;
    expiry_time: number;
}

// The context which stores our access
const AccessContext = createContext<AccessContextType>(null!);

export const useAccess = (): AccessContextType => useContext(AccessContext);
export const useHasAccess = (): boolean => useAccess().token != null

/**
 * Provided for providing the access context to elements
 * further down in the tree. The access provider handles
 * storage of the access token which can be used to
 * sent out requests
 *
 * @param children Children for this element
 * @constructor Creates a new access provider
 */
export const AccessProvider: FunctionComponent<PropsWithChildren> = ({children}) => {

    const [token, setToken] = useState<Token>(localStorage.getItem(LOCAL_STORAGE_KEY));

    useEffect(saveToken, [token]);
    useEffect(() => {
        checkToken().then().catch();
    });

    /**
     * Checks the initial token loaded from local storage
     * to see if the token is still valid or not and will
     * clear the token if its invalid or there was an error
     */
    async function checkToken() {
        if (token == null) return;
        try {
            const response: CheckResponse = await wrapRequest("GET", "auth");
            if (!response.valid) {
                setToken(null);
            }
        } catch (e) {
            setToken(null);
        }
    }

    /**
     * Saves the token into local storage or removes from
     * local store if the value is null
     */
    function saveToken() {
        if (token != null) localStorage.setItem(LOCAL_STORAGE_KEY, token);
        else localStorage.removeItem(LOCAL_STORAGE_KEY);
    }

    /**
     * Wrapper function which wraps the token into the request
     *
     * @param method The request method
     * @param path The request path
     * @param body The optional request body
     */
    function wrapRequest<V>(
        method: RouteMethod,
        path: Route,
        body: any = null
    ): Promise<V> {
        return request<V>(method, path, body, token);
    }

    /**
     * Clears the current authentication token and tells
     * the server to delete the token if it exists
     */
    async function logout() {
        try {
            await wrapRequest("DELETE", "auth");
        } catch (e) {
            console.error(e);
        } finally {
            setToken(null);
        }
    }

    const contextValue: AccessContextType = {token, setToken, request: wrapRequest, logout};
    return <AccessContext.Provider value={contextValue}>{children}</AccessContext.Provider>;
}