use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use actix_web::{get, web};
use actix_web::web::Json;
use serde::{Deserialize, Serialize};

use crate::models::errors::FilesError;
use crate::utils::JsonResult;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(list)
        .service(view);
}

type FilesResult<T> = JsonResult<T, FilesError>;

#[derive(Serialize)]
pub struct DriveFile {
    name: String,
    size: u64,
    permissions: u32,
}

#[derive(Serialize)]
pub struct DrivePath {
    name: String,
    permissions: u32,
}

#[derive(Deserialize)]
pub struct ListRequest {
    path: String,
    drive_path: String,
}

#[derive(Serialize)]
pub struct ListResponse {
    drive_path: String,
    folders: Vec<DrivePath>,
    files: Vec<DriveFile>,
}

pub async fn get_files_at(
    drive_path: &String,
    path: &String,
) -> Result<ListResponse, FilesError> {
    let full_path = Path::new(drive_path)
        .join(path);
    if !full_path.is_dir() {
        Err(FilesError::NotDirectory)?
    }

    let mut files: Vec<DriveFile> = Vec::new();
    let mut folders: Vec<DrivePath> = Vec::new();

    let read_dir = full_path.read_dir()
        .map_err(|_| FilesError::ReadError)?;

    for dir_entry in read_dir {
        let dir_entry = dir_entry
            .map_err(|_| FilesError::ReadError)?;

        let meta = dir_entry.metadata()
            .map_err(|_| FilesError::ReadError)?;

        let name = dir_entry.file_name()
            .to_str()
            .ok_or(FilesError::ReadError)?
            .to_string();

        let permissions = meta.permissions()
            .mode();
        if meta.is_dir() {
            folders.push(DrivePath {
                name,
                permissions,
            });
        } else {
            files.push(DriveFile {
                name,
                permissions,
                size: meta.len()
            });
        }
    }

    Ok(ListResponse {
        drive_path: drive_path.clone(),
        files,
        folders
    })
}


#[get("/files/list")]
pub async fn list(body: Json<ListRequest>) -> FilesResult<ListResponse> {
    let response = get_files_at(&body.drive_path, &body.path).await?;
    Ok(Json(response))
}


#[derive(Deserialize)]
pub struct ViewRequest {
    path: String,
    drive_path: String,
}

pub async fn read_file(drive: &String, path: &String) -> Result<String, FilesError> {
    Ok(format!("This is the file at {} in {}", path, drive))
}

#[get("/files/view")]
pub async fn view(body: Json<ViewRequest>) -> Result<String, FilesError> {
    let contents = read_file(&body.drive_path, &body.path).await?;
    Ok(contents)
}