use std::{ffi::OsString, fs, io::Error, path::Path};

use crate::{config, utilities};

pub fn handle_kidnap(target: OsString, kill: bool, verbose: bool) -> Result<(), Error> {
    let path_to_basement = Path::new(config::basement_folder);
    fs::create_dir_all(path_to_basement)?;

    let path_to_target = Path::new(&target);

    match kill {
        false => utilities::move_files(path_to_target, path_to_basement, verbose)?,
        true => utilities::delete_files(path_to_target, verbose)?,
    };

    Ok(())
}

// FIX
pub fn list_kidnapped(recursive: bool) -> Result<(), Error> {
    let path_to_basement = Path::new(config::basement_folder);
    fs::create_dir_all(path_to_basement)?;

    for maybe_file in fs::read_dir(path_to_basement).expect("This is a folder by definition") {
        let file = maybe_file?;

        match file.path().is_file() {
            true => println!("{:?}", file.file_name()),
            false => {
                if recursive {
                    recursive_print(&file.path())?
                }
            }
        }
    }

    Ok(())
}

fn recursive_print(path: &Path) -> Result<(), Error> {
    match path.is_file() {
        true => println!("{:?}", path.file_name()),
        false => {
            for file in fs::read_dir(path).expect("Already checked that this is a folder") {
                recursive_print(&file?.path())?;
            }
        }
    };

    Ok(())
}
