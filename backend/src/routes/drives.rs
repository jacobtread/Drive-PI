use crate::define_routes;
use crate::models::drives::{DrivesResponse, MountRequest};
use crate::models::errors::DrivesError;
use crate::utils::drives::{get_drive_list, mount_drive, unmount_drive};
use crate::utils::{ok_json, ok_json_empty, JsonResult};
use actix_web::web::Json;
use actix_web::{delete, get, post, web};

define_routes!(list, unmount, mount);

type DrivesResult<T> = JsonResult<T, DrivesError>;
type DrivesResultEmpty = DrivesResult<()>;

#[get("/drives")]
pub async fn list() -> DrivesResult<DrivesResponse> {
    let drives = get_drive_list()?;
    ok_json(drives)
}

#[post("/drives")]
pub async fn mount(body: Json<MountRequest>) -> DrivesResultEmpty {
    mount_drive(&body.path, &body.name)?;
    ok_json_empty()
}

#[delete("/drives")]
pub async fn unmount(body: Json<MountRequest>) -> DrivesResultEmpty {
    unmount_drive(&body.path, &body.name)?;
    ok_json_empty()
}
