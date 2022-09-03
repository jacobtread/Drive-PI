use serde::{Deserialize, Serialize};

pub type DriveVec = Vec<Drive>;

#[derive(Serialize)]
pub struct DrivesResponse {
    pub drives: Vec<Drive>,
    pub mount_root: String,
}

#[derive(Deserialize, Serialize)]
pub struct Drive {
    // Filesystem UUID (e.g. 21c89e37-a0aa-48bc-aead-cec8d9a8e8cc)
    pub uuid: String,
    // Device name (e.g. sda1)
    pub name: String,
    // Filesystem label (e.g. USB Drive)
    pub label: String,
    // Path to device node (e.g. /dev/sda1)
    pub path: String,
    // Mounted file system path (e.g. /run/media/DRIVE)
    // this is None if not mounted
    #[serde(rename(deserialize = "mountpoint"))]
    pub mount: Option<String>,
    // Filesystem capacity. None if file system not mounted
    #[serde(rename(deserialize = "fssize"))]
    pub size: Option<String>,
    // Filesystem used size. None if file system not mounted
    #[serde(rename(deserialize = "fsused"))]
    pub used: Option<String>,
    // Filesystem mount mode (e.g. brw-rw----)
    pub mode: String,
}

#[derive(Deserialize)]
pub struct MountRequest {
    pub path: String,
    pub name: String,
}
