// Base url for accessing the API (e.g. http://localhost:8080)
import { createContext, FunctionComponent, PropsWithChildren, useContext, useEffect, useState } from "react";
import { request, RequestData } from "../api/request";
import { Navigate, Outlet, useLocation } from "react-router-dom";

const LOCAL_STORAGE_KEY = "drivepi_token"

export type Token = string | null;

interface AccessContextType {
    token: Token;
    setToken: (token: Token) => void;
    request: <V>(requestData: RequestData) => Promise<V>
}

// The context which stores our access
const AccessContext = createContext<AccessContextType>(null!)

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

    const [token, setToken] = useState<Token>(localStorage.getItem(LOCAL_STORAGE_KEY));

    useEffect(saveToken, [token])

    /**
     * Saves the token into local storage or removes from
     * local store if the value is null
     */
    function saveToken() {
        if (token != null) localStorage.setItem(LOCAL_STORAGE_KEY, token)
        else localStorage.removeItem(LOCAL_STORAGE_KEY)
    }

    /**
     * Wrapper function which wraps the token into the request
     *
     * @param requestData The request data.
     */
    function wrapRequest<V>(requestData: RequestData): Promise<V> {
        return request<V>(requestData, token)
    }

    const contextValue: AccessContextType = {token, setToken, request: wrapRequest}
    return <AccessContext.Provider value={contextValue}>{children}</AccessContext.Provider>
}

export const RequireAccess: FunctionComponent = () => {
    const {token} = useAccess()
    const location = useLocation()
    if (token == null) {
        return <Navigate to="/auth" state={{from: location}} replace/>
    } else {
        return <Outlet/>;
    }
}