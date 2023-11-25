use crate::utilities;
use std::{ffi::OsString, io::Error, path::Path};

pub fn free(target: OsString, new_location: Option<OsString>) -> Result<(), Error> {
    let path_to_basement = utilities::get_absolute_path_to_basement()?;

    let path_to_target = Path::new(&target);

    let target_name = path_to_target.file_name().unwrap().to_str().unwrap();

    let path_to_new_location = match new_location {
        Some(loc) => Path::new(&loc).to_path_buf(),
        None => std::env::current_dir()?.join(Path::new(target_name)),
    };

    let file_to_free = path_to_basement.join(path_to_target);

    utilities::move_all_files(&file_to_free, &path_to_new_location)?;

    println!(
        "I freed {} as {}",
        target_name,
        path_to_new_location.to_str().unwrap()
    );

    Ok(())
}
