use crate::error::Result;
use std::{
    env, fs, ops::Deref,
    path::{ Path, PathBuf }
};


/// A tempdir handle
#[derive(Debug)]
pub struct Tempdir {
    /// The path of the managed directory
    path: PathBuf
}
impl Tempdir {
    /// Manages an existing directory as temp directory (i.e. it will be deleted if this instance is dropped)
    pub(in crate::packagedir) fn with_path<T>(path: T) -> Result<Self> where T: AsRef<Path> {
        // Test if the path exists
        let path = path.as_ref().to_path_buf();
        if !path.is_dir() {
            Err(eio!("No such directory: {:?}", path))?;
        }

        Ok(Self { path })
    }

    /// Returns the path to the tempdir
    pub fn path(&self) -> &Path {
        &self.path
    }
}
impl Drop for Tempdir {
    fn drop(&mut self) {
        // Delete the directory
        if let Err(err) = fs::remove_dir_all(&self.path) {
            eprintln!("Failed to delete temp directory: {:?} ({})", &self.path, err);
        }
    }
}


/// The package directory
pub struct Packagedir {
    /// The path of the package dir
    path: PathBuf
}
impl Packagedir {
    /// Creates a new packagedir within the home directory
    pub fn new() -> Result<Self> {
        let home = env::var("HOME").map_err(|e| eio!("Failed to get home directory ({})", e))?;
        let path = Path::new(&home).join(".ezinstall");
        Self::with_path(path)
    }
    /// Creates a new packagedir with the given path
    pub fn with_path<T>(path: T) -> Result<Self> where T: AsRef<Path> {
        // Create the directory if necessary
        let path = path.as_ref().to_path_buf();
        if !path.is_dir() {
            fs::create_dir_all(&path)?;
        }

        Ok(Self { path })
    }

    /// Creates a tempdir within the package dir
    pub fn tempdir(&self) -> Result<Tempdir> {
        // Create the path and delete an orphaned directory if necessary
        let tempdir_path = self.path.join("tempdir");
        if tempdir_path.is_dir() {
            fs::remove_dir_all(&tempdir_path)?;
        }

        // Create the directory and it's handle
        fs::create_dir(&tempdir_path)?;
        Tempdir::with_path(tempdir_path)
    }
}
impl AsRef<Path> for Packagedir {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}
impl Deref for Packagedir {
    type Target = Path;
    fn deref(&self) -> &Self::Target {
        &self.path
    }
}