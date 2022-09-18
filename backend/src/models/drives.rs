use serde::{Deserialize, Serialize};

/// Structure representing a response which contains a list
/// of drives and the mount root on the file system
#[derive(Serialize)]
pub struct DrivesResponse {
    /// The list of drives
    pub drives: Vec<Drive>,
    /// The location on the file system where drives are mounted
    pub mount_root: String,
}

/// Structure representing a block device
#[derive(Deserialize, Serialize)]
pub struct Drive {
    /// Filesystem UUID (e.g. 21c89e37-a0aa-48bc-aead-cec8d9a8e8cc)
    pub uuid: String,
    /// Device name (e.g. sda1)
    pub name: String,
    /// Filesystem label (e.g. USB Drive)
    pub label: String,
    /// Path to device node (e.g. /dev/sda1)
    pub path: String,
    /// Mounted file system path (e.g. /run/media/DRIVE)
    /// this is None if not mounted
    #[serde(rename(deserialize = "mountpoint"))]
    pub mount: Option<String>,
    /// Filesystem capacity. None if file system not mounted
    #[serde(rename(deserialize = "fssize"))]
    pub size: Option<String>,
    /// Filesystem used size. None if file system not mounted
    #[serde(rename(deserialize = "fsused"))]
    pub used: Option<String>,
    /// Filesystem mount mode (e.g. brw-rw----)
    pub mode: String,
}

/// Structure for a request to mount a drive
#[derive(Deserialize)]
pub struct MountRequest {
    /// The drive path (e.g. /dev/sda1)
    pub path: String,
    /// The name of the folder to mount the drive to
    pub name: String,
}
