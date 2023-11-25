use crate::config;
use std::{
    fs,
    io::Error,
    path::{Path, PathBuf},
};

pub fn move_all_files(source: &PathBuf, destination: &PathBuf) -> Result<(), Error> {
    std::fs::rename(source, destination)
}

pub fn delete_directory(path: PathBuf) -> Result<(), Error> {
    std::fs::remove_dir_all(path)
}

pub fn delete_file(path: PathBuf) -> Result<(), Error> {
    std::fs::remove_file(path)
}

pub fn get_absolute_path_to_basement() -> Result<PathBuf, Error> {
    let path_to_basement = Path::new(config::BASEMENT_FOLDER)
        .to_path_buf()
        .canonicalize()?;

    fs::create_dir_all(&path_to_basement)?;

    Ok(path_to_basement)
}
