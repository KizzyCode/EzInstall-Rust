use crate::{ error::Result, tar::tar_impl::Tar };
use ezexec::{ ExecBuilder, lookup::Binary };
use std::path::Path;


/// A `gnutar` based tar implementation
pub struct TarImpl;
impl Tar for TarImpl {
    fn new() -> Result<Self> {
        let _binary = Binary::find("tar")?;
        Ok(Self)
    }

    fn extract<A, P>(&self, archive: A, into_dir: P) -> Result where A: AsRef<Path>, P: AsRef<Path> {
        // Create strings from the paths
        let archive = archive.as_ref();
        let archive = archive.to_str()
            .ok_or(einval!("Cowardly refusing to process non-UTF-8 path: {:?}", archive))?;

        let into_dir = into_dir.as_ref();
        let into_dir = into_dir.to_str()
            .ok_or(einval!("Cowardly refusing to process non-UTF-8 path: {:?}", into_dir))?;

        // Extract the archive
        let args = ["--extract", "--restrict", "--file", archive, "--directory", into_dir];
        ExecBuilder::with_name("tar", args)?
            .spawn_captured()?
            .wait()?;
        Ok(())
    }
}
