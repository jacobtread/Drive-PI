use actix_web::{delete, get, post, web};
use actix_web::web::Json;
use serde::{Deserialize};
use crate::models::errors::DrivesError;
use crate::utils::drives::{get_drive_list, Drive, mount_drive, unmount_drive};
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
pub struct MountRequest {
    drive_path: String,
    mount_path: String,
}

#[post("/drives")]
pub async fn mount(body: Json<MountRequest>) -> DrivesResult<()> {
    mount_drive(&body.drive_path, &body.mount_path)?;
    Ok(Json(()))
}

#[derive(Deserialize)]
pub struct UnmountRequest {
    drive_path: String,
}

#[delete("/drives")]
pub async fn unmount(body: Json<UnmountRequest>) -> DrivesResult<()> {
    unmount_drive(&body.drive_path)?;
    Ok(Json(()))
}


