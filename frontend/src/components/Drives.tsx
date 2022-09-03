import { FunctionComponent, useEffect, useState } from "react";
import { ErrorResponse, RouteMethod } from "$api/request";
import { useAccess } from "$components/AccessProvider";
import Drive, { DriveAction } from "$components/Drive";
import { DriveItem, DrivesResponse } from "$api/models";

interface Properties {
    selected: DriveItem | null;
    setSelected: (drive: DriveItem | null) => void
}

const Drives: FunctionComponent<Properties> = (properties) => {
    const {request} = useAccess();
    const {selected, setSelected} = properties;

    const [mountRoot, setMountRoot] = useState<string>("");
    const [drives, setDrives] = useState<DriveItem[]>([]);

    /**
     * Function which loads the drives from the backend api
     * using the GET /drives sorts the returned values and
     * updates the drives and mountRoot states
     */
    async function loadDrives() {
        console.log("Loading drives")
        try {
            // Load drives from backend API
            const response: DrivesResponse = await request("GET", "drives");

            // The absolute path to the mounting route for the drives
            // used to determine whether the drive is shared
            const mountRoot: string = response.mount_root;

            // Create sorted copy of the drives sorting mounted drives to the top
            // ensuring that those mounted at the root are ranked highest
            const drives: DriveItem[] = response.drives
                .sort((a, b) => {
                    if (a.mount !== null && b.mount !== null) {
                        const aShared = a.mount.startsWith(mountRoot);
                        const bShared = b.mount.startsWith(mountRoot);

                        // Sort based on shared state if both are mounted
                        // (those who are shared appear earlier in array)
                        return Number(bShared) - Number(aShared);
                    } else {
                        // Sort based on mount state comparison
                        // (those who are mounted appear earlier in array)
                        return Number(b.mount !== null) - Number(a.mount !== null);
                    }
                });

            setMountRoot(mountRoot);
            setDrives(drives);
        } catch (e) {
            const error = e as ErrorResponse
            console.error(`Failed to load drives response: ${error[0]} ${error[1]}`);
        }
    }

    /**
     * Handles executing the underlying drive action on the
     * backend API. This function is provided to the drive
     * components. The drives list is reloaded after the
     * action is complete
     *
     * @param drive The drive the action is for
     * @param action The action to execute
     */
    async function doAction(drive: DriveItem, action: DriveAction) {
        if (action === DriveAction.UNMOUNTING
            && selected !== null && selected.name === drive.name) {
            setSelected(null)
        }
        const actionMethods: Record<DriveAction, RouteMethod> = {
            [DriveAction.SHARING]: "PUT",
            [DriveAction.MOUNTING]: "POST",
            [DriveAction.UNMOUNTING]: "DELETE"
        };
        const method: RouteMethod = actionMethods[action];
        const body = {path: drive.path, name: drive.label};
        // Make action request on backend API
        await request(method, "drives", body);
        // Reload the drives list
        await loadDrives();
    }

    // Load drives list on initial load
    useEffect(() => {
        loadDrives().then()
    }, [])

    return (
        <div className="drives">
            <button onClick={() => loadDrives()} className="button">Refresh</button>
            {drives.map(drive =>
                <Drive
                    key={drive.name}
                    drive={drive}
                    setSelected={setSelected}
                    mountRoot={mountRoot}
                    doAction={doAction}
                />
            )}
        </div>
    );
}

export default Drives