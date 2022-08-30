import { FunctionComponent, useEffect, useState } from "react";
import { Drive } from "$components/Drives";
import { useAccess } from "$components/AccessProvider";

interface Properties {
    drive: Drive | null
}

interface DriveFile {
    name: string;
    size: number;
    permissions: number;
}

interface DrivePath {
    name: string;
    permissions: string;
}

interface ListFilesResponse {
    drive_path: string;
    folders: DrivePath[];
    files: DriveFile[];
}

interface State {
    path: string;
    history: string[];
}

interface DriveState {
    files: DriveFile[];
    folders: DrivePath[];
}

const FolderIcon: FunctionComponent = () => (
    <svg xmlns="http://www.w3.org/2000/svg" height="48" width="48" fill="#999" className="icon folder-icon">
        <path
            d="M7.05 40q-1.2 0-2.1-.925-.9-.925-.9-2.075V11q0-1.15.9-2.075Q5.85 8 7.05 8h14l3 3h17q1.15 0 2.075.925.925.925.925 2.075v23q0 1.15-.925 2.075Q42.2 40 41.05 40Zm0-29v26h34V14H22.8l-3-3H7.05Zm0 0v26Z"/>
    </svg>
)
const FileIcon: FunctionComponent = () => (
    <svg xmlns="http://www.w3.org/2000/svg" height="48" width="48" fill="#999" className="icon file-icon">
        <path
            d="M15.95 35.5h16.1v-3h-16.1Zm0-8.5h16.1v-3h-16.1ZM11 44q-1.2 0-2.1-.9Q8 42.2 8 41V7q0-1.2.9-2.1Q9.8 4 11 4h18.05L40 14.95V41q0 1.2-.9 2.1-.9.9-2.1.9Zm16.55-27.7V7H11v34h26V16.3ZM11 7v9.3V7v34V7Z"/>
    </svg>
)

const FileBrowser: FunctionComponent<Properties> = ({drive}) => {
    const {request} = useAccess()

    const [selected, setSelected] = useState(-1);
    const [path, setPath] = useState("");
    const [driveState, setDriveState] = useState<DriveState>({
        files: [],
        folders: []
    });

    let isRoot = path.length === 0;

    async function getFiles(path: string, drive_path: string) {
        try {
            const {files, folders} = await request<ListFilesResponse>({
                method: "POST",
                path: "files/list",
                body: {path, drive_path}
            })
            setDriveState({files, folders})
        } catch (e) {
            console.error(e)
        }
    }

    const moveBack = () => {
        let slashIndex = path.lastIndexOf('/')
        if (slashIndex == -1 && path.length > 0) {
            moveHome()
        } else {
            setPath(path.substring(0, slashIndex))
        }
    }

    const moveForward = (folder: string) => {
        let lastPath = path;
        if (lastPath.length > 0) {
            setPath(`${lastPath}/${folder}`)
        } else {
            setPath(folder)
        }
    }

    const moveHome = () => setPath("")

    // Effect for loading the files when the path or drive changes
    useEffect(() => {
        if (drive != null) {
            getFiles(path, drive.path)
                .then()
                .catch(console.error)
        }
    }, [path, drive])

    // Effect for clearing the path when the drive changes
    useEffect(() => setPath(""), [drive])

    if (drive == null) {
        return <div className="browser-error">
            <h2 className="browser-error__title">SELECT DRIVE</h2>
            <p className="browser-error__text">Click view on a drive to browse its file system</p>
        </div>
    }

    return (
        <div className="browser">
            <div className="browser__path input">
                <div className="browser__toolbar">
                    <button onClick={moveHome} disabled={isRoot}>Home</button>
                    <button onClick={moveBack} disabled={isRoot}>Back</button>
                </div>
                <input type="text" readOnly={true} className="browser__path__input input__value"/>
            </div>

            <ul className="browser__list">

                {driveState.folders.map((folder, index) => {

                    return (
                        <li key={index}
                            className="browser__item browser__item--folder"
                            onDoubleClick={() => moveForward(folder.name)}
                        >
                            <FolderIcon/>
                            <p className="browser__item__name">{folder.name}</p>
                        </li>
                    )
                })}

                {driveState.files.map((file, index) => {

                    return (
                        <li key={index} className="browser__item browser__item--file ">
                            <FileIcon/>
                            <div>
                                <p className="browser__item__name">{file.name}</p>
                                <span>{file.size} bytes</span>
                            </div>
                        </li>
                    )
                })}
            </ul>
        </div>
    )
}

export default FileBrowser