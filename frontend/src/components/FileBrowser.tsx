import { FunctionComponent, useState } from "react";
import { Drive } from "$components/Drives";

interface Properties {
    drive: Drive | null
}

const FileBrowser: FunctionComponent<Properties> = ({drive}) => {

    const [path, setPath] = useState("")
    const [history, setHistory] = useState([])

    if (drive == null) {
        return <div className="browser-error">
            <h2 className="browser-error__title">SELECT DRIVE</h2>
            <p className="browser-error__text">Click view on a drive to browse its file system</p>
        </div>
    }

    return (
        <div className="browser">
            <div className="browser__path input">
                <button>Home</button>
                <input type="text" readOnly={true} className="browser__path__input input__value"/>
            </div>
            <div className="browser__toolbar">
                <button>Back</button>
                <button>Forward</button>
                <button>Delete</button>
                <button>View</button>
            </div>
        </div>
    )
}

export default FileBrowser