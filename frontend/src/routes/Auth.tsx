import { ChangeEvent, FormEvent, FunctionComponent, useState } from "react";
import { useAccess } from "../components/AccessProvider";
import { request } from "../api/request";
import { useNavigate } from "react-router-dom";

enum State {
    INITIAL,
    LOADING,
    ERROR,
}

interface AuthState {
    state: State;
    username: string;
    password: string;
    error: string;
}

interface AuthResponse {
    token: string;
    expiry_time: string;
}

const Auth: FunctionComponent = () => {
    const navigate = useNavigate()
    const {setToken} = useAccess()
    const [state, setState] = useState<AuthState>({
        state: State.INITIAL,
        username: "",
        password: "",
        error: ""
    })

    /**
     * Handles updating state when an input
     * value changes using the name properties
     * of the input element
     *
     * @param event The input change event
     */
    const onValueChange = (event: ChangeEvent<HTMLInputElement>) => {
        const element = event.target;
        const name = element.name;
        setState({...state, [name]: element.value})
    }

    const onSubmit = (event: FormEvent<HTMLFormElement>) => {
        event.preventDefault()
        tryAuthenticate()
            .then()
            .catch()
    }

    async function tryAuthenticate() {
        setState({...state, state: State.LOADING})
        try {
            const response = await request<AuthResponse>({
                method: "POST",
                path: "auth"
            });

            setToken(response.token)
            navigate("/")
        } catch (e) {
            let error: string;
            if (e instanceof TypeError) {
                error = "Unable to connect to server"
            } else if (typeof e === 'string') {
                error = e;
            } else {
                error = e?.toString() ?? "Unknown Error Occurred"
            }
            console.error(e);
            setState({...state, state: State.ERROR, error})
        }
    }

    return (
        <div className="auth-wrapper">
            <div className="auth">
                <img className="auth__logo" src="/logo.svg" width="85" height="170" alt="Logo"/>
                <form className="auth__content" onSubmit={onSubmit}>
                    <label className="input">Username
                        <input
                            className="input__value"
                            type="text"
                            name="username"
                            onChange={onValueChange}
                        />
                    </label>
                    <label className="input">Password
                        <input
                            className="input__value"
                            type="password"
                            name="password"
                        />
                    </label>
                    <button className="button" type="submit">Login</button>
                </form>
            </div>
        </div>
    )
}

export default Auth