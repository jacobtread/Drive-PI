import { createContext, FunctionComponent, PropsWithChildren, useContext, useState } from "react";
import { request, Route, RouteMethod, Token } from "$api/request";
import { CheckResponse } from "$api/models";
import { useEffectAsync } from "$app/utils";

const LOCAL_STORAGE_KEY: string = "drivepi_token";

interface AccessContextType {
    token: Token;
    setToken: (token: Token) => void;
    request: <V>(method: RouteMethod, path: Route, body?: any) => Promise<V>;
    logout: () => void;
}

// The context which stores our access
const AccessContext = createContext<AccessContextType>(null!);

export const useAccess = (): AccessContextType => useContext(AccessContext);

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

    const [tokenState, setTokenState] = useState<Token>(null);

    // Check token validity on initial load
    useEffectAsync(isValidToken);

    /**
     * Validates the token loaded from local storage by retrieving its
     * status from the backend API
     */
    async function isValidToken() {
        const token: Token = localStorage.getItem(LOCAL_STORAGE_KEY);
        if (token == null) return;
        // Make a GET request to /api/auth with the token from local storage
        return request<CheckResponse>("GET", "auth", null, token)
            .then(response => setToken(response.valid ? token : null))
            .catch(() => setToken(null))
    }

    /**
     * Makes the current authentication token invalid on the
     * server and then clears the token client side.
     */
    async function logout() {
        // Make a DELETE request to /api/auth with the token set to delete it
        return request("DELETE", "auth", null, tokenState)
            .catch(console.error)
            .finally(() => setToken(null));
    }

    /**
     * Wrapper over the set token state to add or remove
     * the token from local storage based on whether the
     * token value is null
     *
     * @param token The token or null to clear
     */
    function setToken(token: Token) {
        setTokenState(token)
        if (token != null) localStorage.setItem(LOCAL_STORAGE_KEY, token)
        else localStorage.removeItem(LOCAL_STORAGE_KEY)
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
        return request<V>(method, path, body, tokenState)
            .catch(error => {
                // Handle not authenticated errors
                if (error === 401) setToken(null)
                return error;
            });
    }

    // Context state to provide to children
    const contextValue: AccessContextType = {
        token: tokenState,
        setToken,
        request: wrapRequest,
        logout
    };
    return <AccessContext.Provider value={contextValue}>{children}</AccessContext.Provider>;
}