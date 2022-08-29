import { FunctionComponent } from "react";
import { useAccess } from "$components/AccessProvider";

const Heading: FunctionComponent = () => {
    const {logout} = useAccess();
    return (
        <div className="heading">
            <img src="/logo-side.svg" alt="Logo" className="heading__logo"/>
            <div>
                <h1 className="heading__title">Drive-PI</h1>
                <button onClick={logout} className="button heading__logout">
                    Logout
                </button>
            </div>

        </div>
    )
}

export default Heading;