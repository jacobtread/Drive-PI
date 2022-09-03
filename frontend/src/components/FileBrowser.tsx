import { FunctionComponent, useEffect, useState } from "react";
import { DriveItem } from "$components/Drives";
import { useAccess } from "$components/AccessProvider";

interface Properties {
    drive: DriveItem | null
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

interface DriveState {
    files: DriveFile[];
    folders: DrivePath[];
}

const HomeIcon: FunctionComponent = () => (
    <svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#999"
         className="icon home-icon">
        <path d="M0 0h24v24H0V0z" fill="none"/>
        <path d="M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8h5z"/>
    </svg>
)

const BackIcon: FunctionComponent = () => (
    <svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#999"
         className="icon home-icon">
        <path d="M0 0h24v24H0V0z" fill="none" opacity=".87"/>
        <path d="M17.51 3.87L15.73 2.1 5.84 12l9.9 9.9 1.77-1.77L9.38 12l8.13-8.13z"/>
    </svg>
)

const FolderIcon: FunctionComponent = () => (
    <svg xmlns="http://www.w3.org/2000/svg" height="32px" viewBox="0 0 24 24" width="32px" fill="#999"
         className="icon home-icon">
        <path d="M0 0h24v24H0V0z" fill="none"/>
        <path d="M10 4H2v16h20V6H12l-2-2z"/>
    </svg>
)
const FileIcon: FunctionComponent = () => (
    <svg xmlns="http://www.w3.org/2000/svg" height="32px" viewBox="0 0 24 24" width="32px" fill="#999"
         className="icon home-icon">
        <path d="M0 0h24v24H0V0z" fill="none"/>
        <path d="M14 2H4v20h16V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z"/>
    </svg>
)

const FileBrowser: FunctionComponent<Properties> = ({drive}) => {
    const {request} = useAccess()

    const [path, setPath] = useState("");
    const [driveState, setDriveState] = useState<DriveState>({
        files: [],
        folders: []
    });

    let isRoot = path.length === 0;

    async function getFiles(path: string, drive_path: string) {
        try {
            const {files, folders}: ListFilesResponse = await request("POST", "files", {
                path,
                drive_path
            });
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
        if (drive != null && drive.mount != null) {
            getFiles(path, drive.mount)
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
                    <button
                        className="button browser__toolbar__button"
                        onClick={moveHome} disabled={isRoot}>
                        <HomeIcon/>
                    </button>
                    <button
                        className="button browser__toolbar__button"
                        onClick={moveBack} disabled={isRoot}>
                        <BackIcon/>
                    </button>
                </div>
                <input type="text" readOnly={true} className="browser__path__input input__value" value={path}/>
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
                                <span className="browser__item__size">{file.size} bytes</span>
                            </div>
                        </li>
                    )
                })}
            </ul>
        </div>
    )
}

export default FileBrowser