use std::path::PathBuf;
use std::os::unix::fs::PermissionsExt;

use crate::models::errors::FilesError;
use crate::models::files::{DriveFile, DriveFolder, DriveList};

type FilesResult<T> = Result<T, FilesError>;

/// Retrieves a list of files and folders in the provided mount
/// path.
pub fn get_files_at(
    drive_path: &String,
    path: &String,
) -> FilesResult<DriveList> {
    // TODO: Check full_path is inside mounted dir

    let mut full_path = PathBuf::with_capacity(2);
    full_path.push(drive_path);
    full_path.push(path);

    if !full_path.is_dir() {
        return Err(FilesError::NotDirectory);
    }

    let mut folders = Vec::new();
    let mut files = Vec::new();

    for entry in full_path.read_dir()? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let entry_name = entry.file_name()
            .to_string_lossy()
            .to_string();
        let permissions = metadata.permissions()
            .mode();

        if metadata.is_dir() {
            folders.push(DriveFolder {
                name: entry_name,
                permissions,
            })
        } else {
            files.push(DriveFile {
                name: entry_name,
                size: metadata.len(),
                permissions,
            })
        }
    }

    Ok(DriveList {
        folders,
        files,
    })
}