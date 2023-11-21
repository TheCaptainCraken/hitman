use std::{ffi::OsString, fs, io::Error, path::Path};

use crate::{config, utilities};

pub fn free(target: OsString, new_location: OsString, verbose: bool) -> Result<(), Error> {
    let path_to_basement = Path::new(config::basement_folder);
    fs::create_dir_all(path_to_basement)?;

    let path_to_target = Path::new(&target);

    let path_to_new_location = Path::new(&new_location);

    let file_to_free = path_to_basement.join(path_to_target);

    utilities::move_files(&file_to_free, path_to_new_location, verbose)?;

    Ok(())
}
