use std::fs;
use std::path::Path;
use serde_with::{serde_as, VecSkipError};
use serde::{Deserialize, Serialize};
use std::process::Command;
use log::{error, warn};
use crate::models::errors::DrivesError;
use crate::utils::samba::unshare_drive;

#[serde_as]
#[derive(Deserialize)]
pub struct BlockDevice {
    name: String,
    #[serde_as(as = "Option<VecSkipError<_>>")]
    children: Option<Vec<Drive>>,
}


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
pub struct LSBLKOutput {
    #[serde(rename = "blockdevices")]
    devices: Vec<BlockDevice>,
}

const LSBLK_OUTPUT_CONTENTS: &str = "UUID,NAME,LABEL,PATH,MOUNTPOINT,FSSIZE,FSUSED,MODE";

type DrivesResult<T> = Result<T, DrivesError>;

/// Retrieves a list of mounted and unmounted drives using the lsblk command
/// and returns the result.
pub fn get_drive_list() -> DrivesResult<Vec<Drive>> {
    let output = Command::new("lsblk")
        .args([
            "-J" /* Output result as JSON */,
            "-o", LSBLK_OUTPUT_CONTENTS /* Output contents list*/
        ])
        .output()
        .map_err(|_| DrivesError::SystemError)?
        .stdout;
    let devices = serde_json::from_slice::<LSBLKOutput>(&output)
        .map_err(|err| {
            error!("Failed to parse lsblk output: {}", err);
            DrivesError::ParseError
        })?
        .devices;

    let mut drives = Vec::new();
    for device in devices {
        if device.name.starts_with("loop") {
            continue;
        }
        if let Some(children) = device.children {
            for drive in children {
                drives.push(drive)
            }
        }
    }
    return Ok(drives);
}

const MOUNT_DIR: &str = "mount";

/// Handles mounting drives to local paths relative to the executable
/// drives will be mounted to ./mount/{DRIVE_NAME} this is to avoid
/// permission issues. Mounts drive as Read/Write
pub fn mount_drive(path: &String, name: &String) -> DrivesResult<()> {
    // Ensure the local mounting root point exists or create it
    let mount_dir = Path::new(MOUNT_DIR);
    if !mount_dir.exists() {
        fs::create_dir(mount_dir)
            .map_err(|err| {
                error!("Failed to create mount target parent directory: {}", err);
                DrivesError::MountError
            })?;
    }

    // Ensure the local mounting point exists or create it
    let mount_path = mount_dir
        .join(name);
    if !mount_path.exists() {
        fs::create_dir(&mount_path)
            .map_err(|err| {
                error!("Failed to create mount target directory: {}", err);
                DrivesError::MountError
            })?;
    }

    let mount_path_str = mount_path
        .to_str()
        .ok_or_else(||DrivesError::MountError)?;

    let output = Command::new("mount")
        .args([
            "-o", "rw", /* Mount as Read/Write*/
            path, mount_path_str
        ])
        .output()
        .map_err(|err| {
            error!("Failed to execute mount command: {}", err);
            DrivesError::MountError
        })?;

    let status = output.status;

    if !status.success() {
        let stderr = String::from_utf8(output.stderr)
            .unwrap_or(String::from("Failed to parse stderr"));

        warn!("Failed to mount drive {}", stderr);
        Err(DrivesError::MountError)
    } else {
        Ok(())
    }
}

/// Unmounts the provided drive and removes it from the samba share
pub fn unmount_drive(path: &String) -> DrivesResult<()> {
    let output = Command::new("unmount")
        .args([path])
        .output()
        .map_err(|err| {
            error!("Failed to execute unmount command: {}", err);
            DrivesError::UnmountError
        })?;

    let status = output.status;

    if !status.success() {
        let stderr = String::from_utf8(output.stderr)
            .unwrap_or(String::from("Failed to parse stderr"));

        warn!("Failed to unmount drive {}", stderr);
        Err(DrivesError::UnmountError)
    } else {
        unshare_drive(path);
        Ok(())
    }
}