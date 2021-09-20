mod tar_impl;

use crate::{
    error::Result,
    tar::tar_impl::{ Tar, TarImpl }
};
use std::{
    fs,
    path::{ Path, PathBuf }
};


/// An ezinstall tarball handle
pub struct Tarball {
    /// The path to the tarball
    archive: PathBuf
}  
impl Tarball {
    /// Creates a new tarball handle
    pub fn new<T>(path: T) -> Result<Self> where T: Into<PathBuf> {
        // Ensure that the path exists
        let path = path.into();
        if !path.is_file() {
            Err(eio!("No such archive: {:?}", path))?;
        }

        Ok(Self { archive: path.to_path_buf() })
    }

    /// Extracts the archive into the given directory returns the ezinstall top-level directory
    pub fn extract<T>(&self, working_dir: T) -> Result where T: AsRef<Path> {
        // Ensure that the target directory exists
        let working_dir = working_dir.as_ref();
        if !working_dir.is_dir() {
            Err(eio!("The target directory does not exist: {:?}", working_dir))?;
        }
        
        // Extract the archive
        let tar = TarImpl::new()?;
        tar.extract(&self.archive, working_dir)
    }
    /// Gets the real ezinstall package dir (this is useful if the archive is nested; i.e. if we have a
    /// `ArchiveDir/MyProject/my_files` structure this function will return `ArchiveDir/MyProject`)
    pub fn package_dir<T>(&self, working_dir: T) -> Result<PathBuf> where T: AsRef<Path> {
        // Test if the working directory is already an ezinstall package dir
        let working_dir = working_dir.as_ref();
        if Self::is_package_dir(working_dir) {
            // Apparently our archive is flat
            Ok(working_dir.to_path_buf())
        } else {
            // Test the toplevel dirs within the directory
            for entry in fs::read_dir(working_dir)? {
                // Unwrap the entry
                let entry = entry?;
                let entry_path = entry.path();

                // Check if we have a visible package directory
                let is_valid = entry_path.is_dir()
                    && !entry.file_name().to_string_lossy().starts_with(".")
                    && Self::is_package_dir(&entry_path);
                if is_valid {
                    return Ok(entry_path)
                }
            }
            Err(einval!(r#"Archive is not a valid ezinstall package (missing "install.sh" or "uninstall.sh")"#))
        }
    }

    /// Tests if a directory is an ezinstall package dir (i.e. if it contains the obligatory "install.sh" and
    /// "uninstall.sh" scripts)
    fn is_package_dir<T>(dir: T) -> bool where T: AsRef<Path> {
        let dir = dir.as_ref();
        dir.join("install.sh").is_file() && dir.join("uninstall.sh").is_file()
    }
}
