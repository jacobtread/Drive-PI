import { ChangeEvent, FormEvent, FunctionComponent, useState } from "react";
import { useAccess } from "$components/AccessProvider";
import { request } from "$api/request";

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
    expiry_time: number;
}

const Auth: FunctionComponent = () => {
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
                path: "auth",
                body: {
                    username: state.username,
                    password: state.password
                }
            });

            setToken(response.token)
        } catch (e: any) {
            const [statusCode, error] = e as [number, string];
            let errorText: string
            if (statusCode === 401) {
                errorText = "Incorrect credentials"
            } else {
                errorText = error
            }

            setState({...state, state: State.ERROR, error: errorText})
        }
    }

    return (
        <div className="auth-wrapper">
            <div className="auth">
                <img src="/public/logo-side.svg" alt="Logo" className="auth__logo"/>
                {state.state === State.ERROR && (
                    <p className="auth-error">
                        {state.error}
                    </p>
                )}
                <form className="auth__content" onSubmit={onSubmit}>
                    <label className="input">Username
                        <input
                            className="input__value"
                            type="text"
                            name="username"
                            onChange={onValueChange}
                            value={state.username}
                            required
                        />
                    </label>
                    <label className="input">Password
                        <input
                            className="input__value"
                            type="password"
                            name="password"
                            onChange={onValueChange}
                            value={state.password}
                            required
                        />
                    </label>
                    <button className="button" type="submit">Login</button>
                </form>
            </div>

        </div>
    )
}

export default Auth;