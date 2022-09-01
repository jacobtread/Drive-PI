use std::fs;
use std::path::Path;
use std::process::Command;

use actix_web::{delete, get, post, web};
use actix_web::web::Json;
use log::warn;
use serde::{Deserialize};
use crate::models::errors::DrivesError;
use crate::utils::drives::{get_drive_list, Drive};
use crate::utils::JsonResult;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(list)
        .service(unmount)
        .service(mount);
}

type DrivesResult<T> = JsonResult<T, DrivesError>;


#[get("/drives")]
pub async fn list() -> DrivesResult<Vec<Drive>> {
    let drives = get_drive_list()?;
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

