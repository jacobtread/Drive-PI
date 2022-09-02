use actix_web::{post, web};
use actix_web::web::Json;
use serde::Deserialize;

use crate::models::errors::FilesError;
use crate::utils::files::{DriveList, get_files_at};
use crate::utils::JsonResult;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(list);
}

type FilesResult<T> = JsonResult<T, FilesError>;

#[derive(Deserialize)]
pub struct ListRequest {
    path: String,
    drive_path: String,
}

#[post("/files/list")]
pub async fn list(body: Json<ListRequest>) -> FilesResult<DriveList> {
    let response = get_files_at(&body.drive_path, &body.path)?;
    Ok(Json(response))
}
