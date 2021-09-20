/// A tar implementation
#[cfg_attr(target_os = "freebsd", path = "tar_impl_bsdtar.rs")]
#[cfg_attr(target_os = "macos", path = "tar_impl_bsdtar.rs")]
#[cfg_attr(target_os = "linux", path = "tar_impl_gnutar.rs")]
mod tar_impl_real;

use crate::error::Result;
use std::path::Path;

// Re-export the platform specific implementation
pub use tar_impl_real::TarImpl;


/// A tar implementation
pub trait Tar where Self: Sized {
    /// Creates the platform specific tar implementation
    fn new() -> Result<Self>;

    /// Extracts an archive into the given directory
    fn extract<A, P>(&self, archive: A, into_dir: P) -> Result where A: AsRef<Path>, P: AsRef<Path>;
}
