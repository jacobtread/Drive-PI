use actix_web::{delete, get, web};
use actix_web::web::{Json};
use log::{info, warn};
use serde::Serialize;
use uuid::Uuid;

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
    name: String,
    path: String,
    used: u32,
    capacity: u32,
}

type DrivesResult<T> = JsonResult<T, DrivesError>;

pub async fn get_mounted_drives() -> Result<Vec<Drive>, DrivesError> {
    let mock_drives = vec![
        Drive {
            uuid: Uuid::new_v4().to_string(),
            name: "Example Drive".to_string(),
            path: "/dev/sda1".to_string(),
            used: 524288,
            capacity: 26214400
        },
        Drive {
            uuid: Uuid::new_v4().to_string(),
            name: "Test Drive".to_string(),
            path: "/dev/sda2".to_string(),
            used: 324288,
            capacity: 86214400
        },
        Drive {
            uuid: Uuid::new_v4().to_string(),
            name: "USB Drive".to_string(),
            path: "/dev/sda3".to_string(),
            used: 624288,
            capacity: 6214400
        },
    ];
    Ok(mock_drives)
}

#[get("/drives")]
pub async fn list() -> DrivesResult<Vec<Drive>> {
    let drives = get_mounted_drives().await?;
    Ok(Json(drives))
}


#[derive(Serialize)]
pub struct UnmountResponse {
    uuid: String,
}

#[delete("/drives/{drive}")]
pub async fn unmount(uuid: web::Path<String>) -> DrivesResult<UnmountResponse> {
    if let Ok(uuid) = Uuid::parse_str(&uuid) {
        info!("Unmounting drive {}", uuid.to_string());
        Ok(Json(UnmountResponse {
            uuid: uuid.to_string(),
        }))
    } else {
        warn!("Attempted to unmount invalid drive UUID {}", uuid);
        Err(DrivesError::DriveNotFound)
    }
}