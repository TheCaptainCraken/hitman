use crate::{config, utilities};
use std::{
    ffi::OsString,
    fs::{self, DirEntry},
    io::Error,
    path::Path,
};

pub fn handle_kidnap(target: OsString, kill: bool) -> Result<(), Error> {
    let path_to_basement = Path::new(config::BASEMENT_FOLDER);
    fs::create_dir_all(path_to_basement).expect("User should have permission to access this");

    let current_working_directory = std::env::current_dir()?;

    let path_to_target = current_working_directory.join(Path::new(&target));

    let target_name = path_to_target.file_name().unwrap().to_str().unwrap();

    match kill {
        false => utilities::move_all_files(&path_to_target, &path_to_basement.join(target_name))?,
        true => {
            if path_to_target.is_dir() {
                utilities::delete_directory(path_to_target)?
            } else {
                utilities::delete_file(path_to_target)?
            }
        }
    };
    Ok(())
}

pub fn list_hostages() -> Result<(), Error> {
    println!("This is a complete list of my hostages. You can free or massacre them.");

    let path_to_basement = utilities::get_absolute_path_to_basement()?;

    for maybe_file in fs::read_dir(path_to_basement).expect("This is a folder by definition") {
        print_file(&maybe_file?);
    }

    Ok(())
}
fn print_file(file: &DirEntry) {
    match file.file_name().to_str() {
        Some(s) => println!("- {}", s),
        None => println!("Unprintable file"),
    }
}
