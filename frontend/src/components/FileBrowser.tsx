import { FunctionComponent } from "react";

const FileBrowser: FunctionComponent = () => {
    return (
        <div className="browser">
            <div className="browser__path">
                <input type="text" readOnly={true} className=""/>
            </div>
            <div className="browser__toolbar">

            </div>
        </div>
    )
}

export default FileBrowser