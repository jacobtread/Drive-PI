use std::fs;
use std::path::Path;
use serde_with::{serde_as, VecSkipError};
use serde::Deserialize;
use std::process::Command;
use log::{error, warn};
use crate::models::drives::DriveVec;
use crate::models::errors::DrivesError;

pub const MOUNT_DIR: &str = "mount";
const LSBLK_OUTPUT_CONTENTS: &str = "UUID,NAME,LABEL,PATH,MOUNTPOINT,FSSIZE,FSUSED,MODE";

#[serde_as]
#[derive(Deserialize)]
pub struct BlockDevice {
    name: String,
    #[serde_as(as = "Option<VecSkipError<_>>")]
    children: Option<DriveVec>,
}

#[derive(Deserialize)]
pub struct LSBLKOutput {
    #[serde(rename = "blockdevices")]
    devices: Vec<BlockDevice>,
}

type DrivesResult<T> = Result<T, DrivesError>;
type DrivesResultEmpty = DrivesResult<()>;

/// Retrieves a list of mounted and unmounted drives using the lsblk command
/// and returns the result.
pub fn get_drive_list() -> DrivesResult<DriveVec> {
    let output = Command::new("lsblk")
        .args([
            "-J" /* Output result as JSON */,
            "-o", LSBLK_OUTPUT_CONTENTS /* Output contents list*/
        ])
        .output()
        .map_err(|_| DrivesError::IOError)?
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

/// Handles mounting drives to local paths relative to the executable
/// drives will be mounted to ./mount/{DRIVE_NAME} this is to avoid
/// permission issues. Mounts drive as Read/Write
pub fn mount_drive(path: &String, name: &String) -> DrivesResultEmpty {
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
        .ok_or_else(|| DrivesError::MountError)?;

    let output = Command::new("mount")
        .args([
            "-o", "rw", /* Mount as Read/Write*/
            path, mount_path_str
        ])
        .output()
        .map_err(|err| {
            error!("Failed to execute mount command: {}", err);
            DrivesError::IOError
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
pub fn unmount_drive(path: &String) -> DrivesResultEmpty {
    let output = Command::new("unmount")
        .args([path])
        .output()
        .map_err(|err| {
            error!("Failed to execute unmount command: {}", err);
            DrivesError::IOError
        })?;

    let status = output.status;

    if !status.success() {
        let stderr = String::from_utf8(output.stderr)
            .unwrap_or(String::from("Failed to parse stderr"));

        warn!("Failed to unmount drive {}", stderr);
        Err(DrivesError::UnmountError)
    } else {
        Ok(())
    }
}

/// Unmounts and then remounts drive
pub fn remount_drive(path: &String, name: &String) -> DrivesResultEmpty {
    unmount_drive(path)?;
    mount_drive(path, name)
}