// Optional type (for rust similarity value can be of type or null)
export type Option<T> = T | null;

// Structure for response (GET /auth)
export interface CheckResponse {
    valid: boolean; // Whether to current token is valid
    expiry_time: Option<number>; // The expiry time of the token or null if token is not valid
}

// Structure for response (POST /auth)
export interface AuthResponse {
    token: string; // Token string to authenticate requests with
    expiry_time: number;// The expiry time of the token in milliseconds
}


// Structure for response (GET /drives)
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

// Structure for response (POST /files)
export interface FilesResponse {
    files: DriveFile[];
    folders: DriveFolder[];
}

// Structure represents a file on the server
export interface DriveFile {
    name: string; // Name of the file (e.g. file.txt)
    size: number; // Size of the file in bytes
    permissions: number; // File permissions (e.g. 644)
}

// Structure represents a folder on the server
export interface DriveFolder {
    name: string; // Name of folder (e.g. My Folder)
    permissions: number; // Folder permissions (e.g. 644)
}

