use std::process::Command;

use actix_web::{delete, get, web};
use actix_web::web::Json;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use serde_with::{serde_as, VecSkipError};


use crate::models::errors::DrivesError;
use crate::utils::JsonResult;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(list)
        .service(unmount);
}

#[derive(Serialize)]
pub struct Drive {
    uuid: String,
    fs_name: String,
    name: String,
    path: String,
    used: String,
    capacity: String,
    fs_mode: String,
}

type DrivesResult<T> = JsonResult<T, DrivesError>;

#[serde_as]
#[derive(Deserialize)]
pub struct BlockDevice {
    name: String,
    #[serde_as(as = "Option<VecSkipError<_>>")]
    children: Option<Vec<BlockDeviceChild>>,
}

#[derive(Deserialize)]
pub struct BlockDeviceChild {
    name: String,
    label: String,
    uuid: String,

    #[serde(rename = "mountpoint")]
    path: String,
    #[serde(rename = "fssize")]
    size: String,
    #[serde(rename = "fsused")]
    used: String,
    mode: String,
}

#[derive(Deserialize)]
pub struct LSBLKOutput {
    #[serde(rename = "blockdevices")]
    devices: Vec<BlockDevice>,
}

pub async fn get_drives() -> Result<Vec<Drive>, DrivesError> {
    let raw_output = Command::new("lsblk")
        .args([
            "-J" /* Output as JSON */,
            "-o", "NAME,LABEL,UUID,FSSIZE,FSUSED,MOUNTPOINT,MODE", /* Output contents */
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
                output.push(Drive {
                    uuid: child.uuid,
                    name: child.label,
                    fs_name: child.name,
                    capacity: child.size,
                    used: child.used,
                    path: child.path,
                    fs_mode: child.mode,
                })
            }
        }
    }

    return Ok(output);
}

pub async fn get_mounted_drives() -> Result<Vec<Drive>, DrivesError> {
    let mock_drives = get_drives().await?;
    Ok(mock_drives)
}

#[get("/drives")]
pub async fn list() -> DrivesResult<Vec<Drive>> {
    let drives = get_mounted_drives().await?;
    Ok(Json(drives))
}

#[derive(Deserialize)]
pub struct UnmountRequest {
    uuid: String,
}

#[derive(Serialize)]
pub struct UnmountResponse {
    uuid: String,
}

#[delete("/drives")]
pub async fn unmount(body: Json<UnmountRequest>) -> DrivesResult<UnmountResponse> {
    if let Ok(uuid) = Uuid::parse_str(&body.uuid) {
        info!("Unmounting drive {}", uuid.to_string());
        Ok(Json(UnmountResponse {
            uuid: uuid.to_string(),
        }))
    } else {
        warn!("Attempted to unmount invalid drive UUID {}", body.uuid);
        Err(DrivesError::DriveNotFound)
    }
}

