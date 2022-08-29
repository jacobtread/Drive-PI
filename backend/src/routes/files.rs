use actix_web::{get, web};
use actix_web::web::Json;
use log::info;
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
    size: u32,
    last_modified: u128,
}

#[derive(Serialize)]
pub struct DrivePath {
    name: String,
}

#[derive(Deserialize)]
pub struct ListRequest {
    path: String,
    drive: String,
}

#[derive(Serialize)]
pub struct ListResponse {
    drive: String,
    folders: Vec<String>,
    files: Vec<DriveFile>,
}

pub async fn get_files_at(drive: &String, path: &String) -> Result<ListResponse, FilesError> {
    info!("Getting files at {} in drive {}", path, drive);

    let mock_folders = vec![
        String::from("Example Folder"),
        String::from("Test Folder"),
        String::from("A Folder"),
    ];

    let mock_files = vec![
        DriveFile {
            name: String::from("example.txt"),
            size: 1000000,
            last_modified: 1661578999,
        },
        DriveFile {
            name: String::from("test.txt"),
            size: 4000000,
            last_modified: 1661578999,
        },
        DriveFile {
            name: String::from("file.txt"),
            size: 9000000,
            last_modified: 1661578999,
        },
    ];


    Ok(ListResponse {
        drive: drive.clone(),
        files: mock_files,
        folders: mock_folders,
    })
}

#[get("/files/list")]
pub async fn list(body: Json<ListRequest>) -> FilesResult<ListResponse> {
    let response = get_files_at(&body.drive, &body.path).await?;
    Ok(Json(response))
}


#[derive(Deserialize)]
pub struct ViewRequest {
    path: String,
    drive: String,
}

pub async fn read_file(drive: &String, path: &String) -> Result<String, FilesError> {
    Ok(format!("This is the file at {} in {}", path, drive))
}

#[get("/files/view")]
pub async fn view(body: Json<ViewRequest>) -> Result<String, FilesError> {
    let contents = read_file(&body.drive, &body.path).await?;
    Ok(contents)
}