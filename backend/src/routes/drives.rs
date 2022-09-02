use actix_web::{delete, get, post, put, web};
use actix_web::web::Json;
use crate::models::drives::{MountRequest, UnmountRequest, DriveVec};
use crate::models::errors::DrivesError;
use crate::utils::drives::{get_drive_list, mount_drive, unmount_drive, remount_drive};
use crate::utils::{JsonResult, ok_json, ok_json_empty};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(list)
        .service(unmount)
        .service(mount)
        .service(remount);
}

type DrivesResult<T> = JsonResult<T, DrivesError>;
type DrivesResultEmpty = DrivesResult<()>;

#[get("/drives")]
pub async fn list() -> DrivesResult<DriveVec> {
    let drives = get_drive_list()?;
    ok_json(drives)
}

#[post("/drives")]
pub async fn mount(body: Json<MountRequest>) -> DrivesResultEmpty {
    mount_drive(&body.drive_path, &body.mount_path)?;
    ok_json_empty()
}

#[delete("/drives")]
pub async fn unmount(body: Json<UnmountRequest>) -> DrivesResultEmpty {
    unmount_drive(&body.drive_path)?;
    ok_json_empty()
}

#[put("/drives")]
pub async fn remount(body: Json<MountRequest>) -> DrivesResultEmpty {
    remount_drive(&body.drive_path, &body.mount_path)?;
    ok_json_empty()
}
