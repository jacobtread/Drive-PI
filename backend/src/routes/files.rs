use crate::define_routes;
use actix_web::web::Json;
use actix_web::{post, web};

use crate::models::errors::FilesError;
use crate::models::files::{DriveList, ListRequest};
use crate::utils::files::get_files_at;
use crate::utils::{ok_json, JsonResult};

define_routes!(list);

type FilesResult<T> = JsonResult<T, FilesError>;

#[post("/files")]
pub async fn list(body: Json<ListRequest>) -> FilesResult<DriveList> {
    let response = get_files_at(&body.drive_path, &body.path)?;
    ok_json(response)
}
