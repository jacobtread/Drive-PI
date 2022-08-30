import { FunctionComponent, ReactElement, useEffect, useState } from "react";
import { useAccess } from "$components/AccessProvider";

interface Properties {
    selected: Drive | null,
    setSelected: (drive: Drive | null) => void
}

export interface Drive {
    uuid: string; // fs UUID
    name: string; // Device name
    label: string; // fs label
    path: string; // Path to device node
    mount: string | null; // fs mount point
    size: string | null; // capacity
    used: string | null; // Used size
    mode: string; // fs mount mode
}

const Drives: FunctionComponent<Properties> = ({selected, setSelected}) => {
    const {request} = useAccess();

    const [drives, setDrives] = useState<Drive[]>([])
    const [unmounting, setUnmounting] = useState<string[]>([])

    useEffect(() => {
        getDrives()
            .then()
            .catch()
    }, [])

    async function getDrives() {
        console.log('Loading Drives')
        await request<Drive[]>({
            method: "GET",
            path: "drives"
        })
            .then(drives => {
                let sortedDrives = drives.sort(drive => drive.mount === null ? 0 : -1);
                setDrives(sortedDrives)
            })
            .catch(console.error)
    }

    async function unmount(drive: Drive) {
        if (selected !== null && selected.uuid === drive.uuid) {
            setSelected(null)
        }

        setUnmounting(values => [...values, drive.uuid]);

        try {
            console.log('Unmounting drive', drive)

            await request({
                method: "DELETE",
                path: "drives",
                body: {drive_path: drive.path}
            })

            await getDrives()
            setUnmounting(values => values.filter(value => value !== drive.uuid));
        } catch (e) {
            console.error(e)
        }
    }

    async function mount(drive: Drive) {
        setUnmounting(values => [...values, drive.uuid]);

        try {
            console.log('Mounting drive', drive)

            await request({
                method: "POST",
                path: "drives",
                body: {
                    drive_path: drive.path,
                    mount_path: drive.name
                }
            })
            await getDrives()
            setUnmounting(values => values.filter(value => value !== drive.uuid));
        } catch (e) {
            console.error(e)
        }
    }

    return (
        <div className="drives">
            {drives.map((drive, index) => {
                let actionText: string;
                let actions: ReactElement;
                if (drive.mount != null) {
                    actionText = "Unmounting"
                    actions = (
                        <div className="drive__actions">
                            <button className="button" onClick={() => setSelected(drive)}>
                                View
                            </button>
                            <button className="button" onClick={() => unmount(drive)}>
                                Unmount
                            </button>
                        </div>
                    )
                } else {
                    actionText = "Mounting"
                    actions = (
                        <div className="drive__actions">
                            <button className="button" onClick={() => mount(drive)}>
                                Mount
                            </button>
                        </div>
                    )
                }

                return (
                    <div key={index} className="drive">
                        {unmounting.indexOf(drive.uuid) != -1 && (
                            <div className="drive__unmounting">
                                <p className="drive__unmounting__text">{actionText} {drive.label} ({drive.path})</p>
                                <div className="loader"></div>
                            </div>
                        )}
                        <img src="/usb.svg" alt="" height={64} className="drive__icon"/>
                        <div className="drive__details">
                            <p className="drive__name">{drive.label} <span>{drive.name}</span></p>
                            <p className="drive__cap">Using <span>{drive.used}</span> of <span>{drive.size}</span>
                            </p>
                            <p className="drive__mount">Mounted at {drive.mount}</p>
                        </div>
                        <div className="drive__actions-wrapper">
                            {actions}
                        </div>
                    </div>
                )
            })}
        </div>
    )
}

export default Drives