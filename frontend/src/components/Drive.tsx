import { FunctionComponent, ReactElement, useState } from "react";
import { ErrorResponse } from "$api/request";
import USB from "$assets/images/usb.svg";
import { DriveItem } from "$api/models";


// Enum for representing the different states a drive could be in
// after an action is triggered on it. Mapped to their http method
export enum DriveAction {
    UNMOUNTING = "Unmounting",
    MOUNTING = "Mounting",
    SHARING = "Sharing"
}

interface Properties {
    drive: DriveItem;
    mountRoot: string;
    setSelected: (drive: DriveItem | null) => void;
    doAction: (drive: DriveItem, action: DriveAction) => Promise<void>;
}

const Drive: FunctionComponent<Properties> = (properties) => {
    const {drive, mountRoot, setSelected, doAction} = properties

    const [action, setAction] = useState<DriveAction | null>(null);
    const [error, setError] = useState<string | null>(null)

    const isMounted = drive.mount !== null;
    const isShared = drive.mount !== null && drive.mount.startsWith(mountRoot);

    async function tryDoAction(action: DriveAction) {
        try {
            // Set the drive action
            setAction(action);
            await doAction(drive, action);
        } catch (e) {
            const error = e as ErrorResponse
            console.error(`Failed drive action: ${error[0]} ${error[1]}`);
            setError(error[1]);
        } finally {
            // Clear the current drive action
            setAction(null);
        }
    }

    let message: ReactElement | "" = "";
    if (error !== null) {
        message = (
            <div className="drive__message">
                <p className="drive__message__text">{error}</p>
            </div>
        )
    } else if (action !== null) {
        message = (
            <div className="drive__message">
                <p className="drive__message__text">{action}</p>
                <div className="loader"></div>
            </div>
        )
    }


    if (isShared) {
        return <div className="drive">
            {message}
            <img src={USB} alt="" height={64} className="drive__icon"/>
            <div className="drive__details">
                <p className="drive__name">{drive.label} <span className="drive__name__sub">{drive.name}</span></p>
                <p className="drive__cap">Using <span>{drive.used}</span> of <span>{drive.size}</span></p>
                <p className="drive__mount">Mounted at {drive.mount}</p>
            </div>
            <div className="drive__actions-wrapper">
                <div className="drive__actions">
                    <button className="button" onClick={() => setSelected(drive)}>
                        View
                    </button>
                    <button className="button" onClick={() => tryDoAction(DriveAction.UNMOUNTING)}>
                        Unmount
                    </button>
                </div>
            </div>
        </div>
    } else if (isMounted) {
        return <div className="drive">
            {message}
            <img src={USB} alt="" height={64} className="drive__icon"/>
            <div className="drive__details">
                <p className="drive__name">{drive.label} <span className="drive__name__sub">{drive.name}</span></p>
                <p className="drive__cap">Using <span>{drive.used}</span> of <span>{drive.size}</span></p>
                <p className="drive__mount">Drive not being shared</p>
            </div>
            <div className="drive__actions-wrapper">
                <div className="drive__actions">
                    <button className="button" onClick={() => tryDoAction(DriveAction.SHARING)}>
                        Share
                    </button>
                </div>
            </div>
        </div>
    } else {
        return <div className="drive">
            {message}
            <img src={USB} alt="" height={64} className="drive__icon"/>
            <div className="drive__details">
                <p className="drive__name">{drive.label} <span className="drive__name__sub">{drive.name}</span></p>
                <p className="drive__mount">Not Mounted</p>
            </div>
            <div className="drive__actions-wrapper">
                <div className="drive__actions">
                    <button className="button" onClick={() => tryDoAction(DriveAction.MOUNTING)}>
                        Mount
                    </button>
                </div>
            </div>
        </div>
    }
}

export default Drive;