use std::{fs, io::Error, path::Path};

use crate::{config, utilities};

pub fn handle_massacre(verbose: bool) -> Result<(), Error> {
    let path_to_basement = Path::new(config::basement_folder);
    fs::create_dir_all(path_to_basement)?;
    utilities::delete_files(path_to_basement, verbose)?;
    Ok(())
}
