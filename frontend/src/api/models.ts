type Option<T> = T | null;

// Structure for responses (GET /drives)
export interface DrivesResponse {
    drives: DriveItem[];
    mount_root: string;
}

// Structure represents a drive / partition on the server
export interface DriveItem {
    uuid: string; // Filesystem UUID for the device
    name: string; // Device name (e.g. sda1)
    label: string; // Device Label (e.g. My Drive)
    path: string; // Device Path (e.g. /dev/sda1)

    mount: Option<string>; // Mounted fs path (e.g. /mnt/sda1)
    size: Option<string>; // Filesystem capacity
    used: Option<string>; // Filesystem used size

    mode: string; // Filesystem mount mode (e.g. brw-rw----)
}
