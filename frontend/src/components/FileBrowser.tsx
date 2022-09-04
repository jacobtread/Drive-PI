import { FunctionComponent, useEffect, useState } from "react";
import { useAccess } from "$components/AccessProvider";
import { DriveFile, DriveFolder, DriveItem, FilesResponse } from "$api/models";
import { BackIcon, FileIcon, FolderIcon, HomeIcon } from "$components/Icons";

interface Properties {
    drive: DriveItem | null;
}

interface DriveState {
    files: DriveFile[];
    folders: DriveFolder[];
}

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
            const response: FilesResponse =
                await request("POST", "files", {path, drive_path});
            setDriveState({
                files: response.files,
                folders: response.folders
            })
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