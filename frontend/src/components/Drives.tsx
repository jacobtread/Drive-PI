import { FunctionComponent, useEffect, useState } from "react";
import { useAccess } from "$components/AccessProvider";

interface Drive {
    uuid: string;
    name: string;
    path: string;
    used: number; // Usage in KB
    capacity: number;
    unmounting: boolean;
}

function getUnitText(value: number) {
    if (Math.floor(value / 1073741824) >= 1) {
        let tb = Math.floor(value / 1073741824)
        return `${tb}TB`;
    } else if (Math.floor(value / 1048576) >= 1) {
        let gb = Math.floor(value / 1048576)
        return `${gb}GB`;
    } else if (Math.floor(value / 1024) >= 1) {
        let mb = Math.floor(value / 1024)
        return `${mb}MB`;
    } else {
        return `${value}KB`;
    }
}

const Drives: FunctionComponent = () => {
    const {request} = useAccess();

    const [drives, setDrives] = useState<Drive[]>([])
    const [unmounting, setUnmounting] = useState<string[]>([])

    useEffect(getDrives, [])

    function getDrives() {
        console.log('Loading Drives')
        request<Drive[]>({
            method: "GET",
            path: "drives"
        })
            .then(drives => setDrives(drives))
            .catch(console.error)
    }

    async function unmount(drive: Drive) {
        setUnmounting(values => [...values, drive.uuid]);

        try {
            console.log('Unmounting drive', drive)

            await request({
                method: "DELETE",
                path: "drives",
                body: {
                    uuid: drive.uuid
                }
            })

            setUnmounting(values => values.filter(value => value !== drive.uuid));
            setDrives(drives => drives.filter(value => value.uuid !== drive.uuid));
        } catch (e) {
            console.error(e)
        }
    }

    return (
        <div className="drives">
            {drives.map((drive, index) => {
                let capacity = getUnitText(drive.capacity);
                let used = getUnitText(drive.used);

                return (
                    <div key={index} className="drive">
                        {unmounting.indexOf(drive.uuid) != -1 && (
                            <div className="drive__unmounting">
                                <p className="drive__unmounting__text">Unmounting {drive.name}</p>
                                <div className="loader"></div>
                            </div>
                        )}
                        <img src="/usb.svg" alt="" height={64} className="drive__icon"/>
                        <div className="drive__details">
                            <p className="drive__name">{drive.name}</p>
                            <p className="drive__cap">Using <span>{used}</span> of <span>{capacity}</span></p>
                            <p className="drive__mount">Mounted at {drive.path}</p>
                        </div>
                        <div className="drive__actions-wrapper">
                            <div className="drive__actions">
                                <button className="button">
                                    View
                                </button>
                                <button className="button" onClick={() => unmount(drive)}>
                                    Unmount
                                </button>
                            </div>
                        </div>
                    </div>
                )
            })}
        </div>
    )
}

export default Drives