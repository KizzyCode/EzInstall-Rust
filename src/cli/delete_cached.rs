use crate::{ error::Result, packagedir::Packagedir };
use std::fs;


/// Forcibly deletes a cached archive
pub fn exec<T>(name: T) -> Result where T: AsRef<str> {
    // Get the path to the cached archive
    let packagedir = Packagedir::new()?;
    let path = packagedir.join(name.as_ref());

    // Delete the file
    fs::remove_file(path)?;
    Ok(())
}
