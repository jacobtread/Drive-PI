use std::path::Path;
use std::os::unix::fs::PermissionsExt;

use crate::models::errors::FilesError;
use crate::models::files::{DriveFile, DriveFolder, DriveList};
use crate::utils::drives::get_mount_root;

type FilesResult<T> = Result<T, FilesError>;

/// Retrieves a list of files and folders in the provided mount
/// path.
pub fn get_files_at(
    drive_path: &String,
    path: &String,
) -> FilesResult<DriveList> {
    let mount_root = get_mount_root()?;

    let full_path = Path::new(drive_path)
        .join(path)
        .canonicalize()?;

    // Ensure the directory is within the mount root directory
    if !full_path.starts_with(mount_root) {
        return Err(FilesError::OutsideMountRoot)
    }

    // Ensure the path is actually a directory and not a file
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