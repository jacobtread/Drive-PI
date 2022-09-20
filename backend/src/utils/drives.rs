use crate::models::drives::{Drive, DrivesResponse};
use crate::models::errors::DrivesError;
use log::{error, info, warn};
use serde::Deserialize;
use serde_with::{serde_as, VecSkipError};
use std::fs::{create_dir, remove_dir};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, io};

pub const MOUNT_DIR: &str = "mount";
const LSBLK_OUTPUT_CONTENTS: &str = "UUID,NAME,LABEL,PATH,MOUNTPOINT,FSSIZE,FSUSED,MODE";

#[serde_as]
#[derive(Deserialize)]
pub struct BlockDevice {
    name: String,
    #[serde_as(as = "Option<VecSkipError<_>>")]
    children: Option<Vec<Drive>>,
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
pub fn get_drive_list() -> DrivesResult<DrivesResponse> {
    let mut command = Command::new("lsblk");
    command.args([
        "-J", /* Output the results as JSON */
        "-o",
        LSBLK_OUTPUT_CONTENTS, /* List of columns to add to output */
    ]);
    let output = command.output().map_err(|err| {
        error!("Failed to execute lsblk command: {}", err);
        DrivesError::IOError
    })?;
    let stdout = output.stdout;
    let parsed = serde_json::from_slice::<LSBLKOutput>(&stdout).map_err(|err| {
        error!("Failed to parse lsblk output: {}", err);
        DrivesError::ParseError
    })?;
    let devices = parsed.devices;
    let mut drives = Vec::new();
    for device in devices {
        if device.name.starts_with("loop") {
            continue;
        }
        if let Some(children) = device.children {
            for drive in children {
                if let Some(mount) = &drive.mount {
                    // Exclude system parts
                    if mount == "/" || mount.starts_with("/boot") {
                        continue;
                    }
                }

                drives.push(drive)
            }
        }
    }
    let mount_dir = get_mount_root()?;
    let mount_root = mount_dir.to_string_lossy().to_string();

    return Ok(DrivesResponse { drives, mount_root });
}

pub fn get_mount_root() -> io::Result<PathBuf> {
    let mount_path = Path::new(MOUNT_DIR);
    if !mount_path.exists() {
        create_dir(mount_path)?;
    }
    mount_path.canonicalize()
}

/// Handles mounting drives to local paths relative to the executable
/// drives will be mounted to ./mount/{DRIVE_NAME} this is to avoid
/// permission issues. Mounts drive as Read/Write
pub fn mount_drive(path: &String, name: &String) -> DrivesResultEmpty {
    // Ensure the local mounting root point exists or create it
    let mount_dir = Path::new(MOUNT_DIR);
    if !mount_dir.exists() {
        fs::create_dir(mount_dir).map_err(|err| {
            error!("Failed to create mount target parent directory: {}", err);
            DrivesError::MountError
        })?;
    }

    // Ensure the local mounting point exists or create it
    let mount_path = mount_dir.join(name);
    if !mount_path.exists() {
        fs::create_dir(&mount_path).map_err(|err| {
            error!("Failed to create mount target directory: {}", err);
            DrivesError::MountError
        })?;
    }

    let mount_path_str = mount_path.to_str().ok_or_else(|| DrivesError::MountError)?;

    let output = Command::new("mount")
        .args([
            "-o",
            "rw", /* Mount as Read/Write*/
            path,
            mount_path_str,
        ])
        .output()
        .map_err(|err| {
            error!("Failed to execute mount command: {}", err);
            DrivesError::IOError
        })?;

    let status = output.status;

    if !status.success() {
        let stderr =
            String::from_utf8(output.stderr).unwrap_or(String::from("Failed to parse stderr"));

        warn!("Failed to mount drive {}", stderr);

        Err(DrivesError::MountError)
    } else {
        chown_mounted_drive(mount_path_str)?;

        Ok(())
    }
}

fn chown_mounted_drive(path: &str) -> DrivesResultEmpty {
    let output = Command::new("chmod")
        .args(["a+rw" /* Read/Write*/, path])
        .output()
        .map_err(|err| {
            error!("Failed to execute chmod command: {}", err);
            DrivesError::IOError
        })?;

    let status = output.status;

    if !status.success() {
        let stderr =
            String::from_utf8(output.stderr).unwrap_or(String::from("Failed to parse stderr"));

        warn!("Failed to change mounted drive permissions {}", stderr);
        Err(DrivesError::MountError)
    } else {
        Ok(())
    }
}

/// Unmounts the provided drive and removes it from the samba share
pub fn unmount_drive(path: &String, name: &String) -> DrivesResultEmpty {
    let output = Command::new("umount")
        .args([path])
        .output()
        .map_err(|err| {
            error!("Failed to execute unmount on {} command: {}", path, err);
            DrivesError::IOError
        })?;

    let mount_dir = get_mount_root()?;
    let mount_path = mount_dir.join(name);
    if mount_path.exists() {
        let is_empty = fs::read_dir(&mount_path)?.next().is_none();

        if is_empty {
            remove_dir(&mount_path)?;
            let mount_path_str = mount_path.to_str().ok_or_else(|| DrivesError::IOError)?;
            info!("Removed directory of unmounted drive: {}", mount_path_str)
        }
    }

    let status = output.status;

    if !status.success() {
        let stderr =
            String::from_utf8(output.stderr).unwrap_or(String::from("Failed to parse stderr"));

        if stderr.contains("target is busy") {
            Err(DrivesError::TargetBusy)
        } else {
            warn!("Failed to unmount drive {}", stderr);
            Err(DrivesError::UnmountError)
        }
    } else {
        Ok(())
    }
}
