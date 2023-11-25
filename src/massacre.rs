use crate::utilities;
use std::io::Error;

pub fn handle_massacre() -> Result<(), Error> {
    let path_to_basement = utilities::get_absolute_path_to_basement()?;

    utilities::delete_directory(path_to_basement.to_path_buf())?;

    println!("I have killed everyone of my innocent hosteges. They suffered.");
    Ok(())
}
