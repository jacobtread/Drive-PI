use serde::{Deserialize, Serialize};

pub type DriveVec = Vec<Drive>;

#[derive(Deserialize, Serialize)]
pub struct Drive {
    // Filesystem UUID (e.g. 21c89e37-a0aa-48bc-aead-cec8d9a8e8cc)
    uuid: String,
    // Device name (e.g. sda1)
    name: String,
    // Filesystem label (e.g. USB Drive)
    label: String,
    // Path to device node (e.g. /dev/sda1)
    path: String,
    // Mounted file system path (e.g. /run/media/DRIVE)
    // this is None if not mounted
    #[serde(rename(deserialize = "mountpoint"))]
    mount: Option<String>,
    // Filesystem capacity. None if file system not mounted
    #[serde(rename(deserialize = "fssize"))]
    size: Option<String>,
    // Filesystem used size. None if file system not mounted
    #[serde(rename(deserialize = "fsused"))]
    used: Option<String>,
    // Filesystem mount mode (e.g. brw-rw----)
    mode: String,
}

#[derive(Deserialize)]
pub struct MountRequest {
    pub drive_path: String,
    pub mount_path: String,
}

#[derive(Deserialize)]
pub struct UnmountRequest {
    pub drive_path: String,
}