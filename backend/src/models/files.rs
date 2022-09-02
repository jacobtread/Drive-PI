use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ListRequest {
    pub path: String,
    pub drive_path: String,
}

#[derive(Serialize)]
pub struct DriveFile {
    pub name: String,
    pub size: u64,
    pub permissions: u32,
}

#[derive(Serialize)]
pub struct DriveFolder {
    pub name: String,
    pub permissions: u32,
}

#[derive(Serialize)]
pub struct DriveList {
    pub folders: Vec<DriveFolder>,
    pub files: Vec<DriveFile>,
}