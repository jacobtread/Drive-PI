use actix_web::{post, web};
use actix_web::web::Json;
use crate::define_routes;

use crate::models::errors::FilesError;
use crate::models::files::{DriveList, ListRequest};
use crate::utils::files::get_files_at;
use crate::utils::{JsonResult, ok_json};

define_routes!(list);

type FilesResult<T> = JsonResult<T, FilesError>;

#[post("/files")]
pub async fn list(body: Json<ListRequest>) -> FilesResult<DriveList> {
    let response = get_files_at(&body.drive_path, &body.path)?;
    ok_json(response)
}