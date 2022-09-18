use serde::{Deserialize, Serialize};

/// Structure of a request for a list of files on a
/// mounted drive.
#[derive(Deserialize)]
pub struct ListRequest {
    /// The mount drive path
    pub path: String,
    /// The path to the directory relative to the mount path
    pub drive_path: String,
}

/// Structure representing a file stored on a mounted drive
#[derive(Serialize)]
pub struct DriveFile {
    /// The name of the file (e.g. example.txt)
    pub name: String,
    /// The size in bytes of the file
    pub size: u64,
    /// The unix file permissions of the file
    pub permissions: u32,
}

/// Structure representing a folder stored on a mounted drive
#[derive(Serialize)]
pub struct DriveFolder {
    /// The name of the folder (e.g. Example Folder)
    pub name: String,
    /// The unix file permissions for the folder
    pub permissions: u32,
}

/// Structure representing a response that contains a list of
/// files and folders.
#[derive(Serialize)]
pub struct DriveList {
    /// The list of folders
    pub folders: Vec<DriveFolder>,
    /// The list of files
    pub files: Vec<DriveFile>,
}
