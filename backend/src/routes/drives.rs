use std::fs;
use std::path::Path;
use std::process::Command;

use actix_web::{delete, get, post, web};
use actix_web::web::Json;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, VecSkipError};

use crate::models::errors::DrivesError;
use crate::utils::JsonResult;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(list)
        .service(unmount)
        .service(mount);
}

type DrivesResult<T> = JsonResult<T, DrivesError>;

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
    #[serde(rename(deserialize="mountpoint"))]
    mount: Option<String>,
    // Filesystem capacity. None if file system not mounted
    #[serde(rename(deserialize="fssize"))]
    size: Option<String>,
    // Filesystem used size. None if file system not mounted
    #[serde(rename(deserialize="fsused"))]
    used: Option<String>,

    // Filesystem mount mode (e.g. brw-rw----)
    mode: String,
}

#[derive(Deserialize)]
pub struct LSBLKOutput {
    #[serde(rename = "blockdevices")]
    devices: Vec<BlockDevice>,
}

pub fn get_drives() -> Result<Vec<Drive>, DrivesError> {
    let raw_output = Command::new("lsblk")
        .args([
            "-J" /* Output as JSON */,
            "-o", "UUID,NAME,LABEL,PATH,MOUNTPOINT,FSSIZE,FSUSED,MODE", /* Output contents */
        ])
        .output()
        .map_err(|_| DrivesError::SystemError)?
        .stdout;

    let result = serde_json::from_slice::<LSBLKOutput>(&raw_output)
        .map_err(|e| {
            warn!("Failed to parse lsblk output: {}", e);


            DrivesError::ParseError
        })?;

    let devices = result.devices;

    let mut output: Vec<Drive> = Vec::new();

    for block_device in devices {
        // Ignore loop devices
        if block_device.name.starts_with("loop") {
            continue;
        }

        if let Some(children) = block_device.children {
            for child in children {
                output.push(child)
            }
        }
    }

    return Ok(output);
}

#[get("/drives")]
pub async fn list() -> DrivesResult<Vec<Drive>> {
    let drives = get_drives()?;
    Ok(Json(drives))
}

#[derive(Deserialize)]
pub struct UnmountRequest {
    drive_path: String,
}

#[delete("/drives")]
pub async fn unmount(body: Json<UnmountRequest>) -> DrivesResult<()> {
    let output = Command::new("umount")
        .args([body.drive_path.clone()])
        .output()
        .map_err(|_| DrivesError::UnmountError)?;

    let status = output.status;

    if status.success() {
        Ok(Json(()))
    } else {
        let stderr = String::from_utf8(output.stderr)
            .unwrap_or(String::from("Failed to parse stderr"));

        warn!("Failed to unmount drive {}", stderr);
        Err(DrivesError::UnmountError)
    }
}

#[derive(Deserialize)]
pub struct MountRequest {
    drive_path: String,
    mount_path: String,
}


#[post("/drives")]
pub async fn mount(body: Json<MountRequest>) -> DrivesResult<()> {
    let mount_path = format!("/mnt/{}", body.mount_path);

    let path = Path::new(&mount_path);
    if !path.exists() {
        fs::create_dir(path)
            .map_err(|e| {
                warn!("Unable to create mount dir target {}", e);
                DrivesError::MountError
            })?;
    }

    let output = Command::new("mount")
        .args([body.drive_path.clone(), mount_path])
        .output()
        .map_err(|_| DrivesError::MountError)?;

    let status = output.status;

    if status.success() {
        Ok(Json(()))
    } else {
        let stderr = String::from_utf8(output.stderr)
            .unwrap_or(String::from("Failed to parse stderr"));

        warn!("Failed to mount drive {}", stderr);
        Err(DrivesError::MountError)
    }
}

