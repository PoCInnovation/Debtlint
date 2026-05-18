use std::{fs::File, ops::AddAssign, path::Path };
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FileManagerError {
    #[error("file not found")]
    FileNotFound,
    #[error("Permission denied")]
    PermissionDenied,
    #[error("Invalid path")]
    InvalidPath,
    #[error("io error: {0}")]
    GeneralError(#[from] std::io::Error),
}

pub struct FileManager {}

impl FileManager {
    pub fn list_files(path: String) -> Result<Vec<String>, FileManagerError> {
        let mut vec_file: Vec<String> = Vec::new();
        if !Path::new(&path).exists() {
            return Err(FileManagerError::FileNotFound);
        }
        for entry in std::fs::read_dir(path)? {
            let result = entry?;
            let file_type = result.file_type()?;
            if file_type.is_dir() {
                continue;
            }
            let into_string = result.file_name().into_string().unwrap();
            vec_file.push(into_string);
        }
        Ok(vec_file)
    }
    
    pub fn read_file(path: String) -> Result<String, FileManagerError> {
        let mut content: String =  String::from("");
        let perm = File::options().read(true).open(&path);
        match perm {
            Ok(_) => {},
            Err(_) => { return Err(FileManagerError::PermissionDenied); }
        }
        if !Path::new(&path).exists() {
            return Err(FileManagerError::FileNotFound);
        }
        let entry = std::fs::read_to_string(&path)?;
        content.add_assign(&entry);
        Ok(content)
    }

    pub fn write_file(path: String, content: String) -> Result<(), FileManagerError>
    {
        let mut full_path: Vec<&str> = path.split("/").collect();
        full_path.pop();
    
        if Path::new(&path).exists() {
            let perm = File::options().read(true).open(&path);
            match perm
            {
                Ok(_) => {std::fs::write(&path, content)?;},
                Err(_) => { return Err(FileManagerError::PermissionDenied); }
            }
        }
        else {
            std::fs::create_dir_all(full_path.iter().map(|i| i.to_string()).collect::<String>())?;
            std::fs::write(&path, content)?;
        }
        Ok(())
    }

    pub fn copy_file(src_path: String, dest_path: String) -> Result<(), FileManagerError>
    {
        let content = FileManager::read_file(src_path)?;
        FileManager::write_file(dest_path, content)
    }

}
