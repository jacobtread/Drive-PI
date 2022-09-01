use serde_with::{serde_as, VecSkipError};
use serde::{Deserialize, Serialize};
use std::process::Command;
use log::error;
use crate::models::errors::DrivesError;

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

pub fn get_drive_list() -> Result<Vec<Drive>, DrivesError> {
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
            continue
        }
        if let Some(children) = device.children {
            for drive in children {
                drives.push(drive)
            }
        }
    }
    return Ok(drives)
}