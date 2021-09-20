use crate::{ error::Result, packagedir::Packagedir, fetch::RemoteResource, tar::Tarball };
use ezexec::ExecBuilder;
use std::fs;


/// Uninstalls a package
pub fn exec<T>(url: T) -> Result where T: AsRef<str> {
    // Get the package directory and create a tempdir
    let packagedir = run_verbose! {
        Packagedir::new()? => packagedir,
        eprint!("Opening packages dir... "),
        eprintln!("-> {:?}", packagedir.as_ref())
    };
    let tempdir = packagedir.tempdir()?;

    // Fetch the resource if necessary
    let remote_resource = RemoteResource::new(url)?;
    let archive_path = packagedir.join(remote_resource.local_name());
    if !archive_path.is_file() {
        run_verbose! {
            remote_resource.fetch(&archive_path)?,
            eprint!("Fetching archive... "),
            eprintln!("-> {:?}", archive_path)
        };
    }

    // Extract the archive
    let tarball = Tarball::new(&archive_path)?;
    run_verbose! {
        tarball.extract(tempdir.path())?,
        eprint!("Extracting archive... "),
        eprintln!("-> {:?}", tempdir.path())
    };
    
    // Get the paths
    let package_dir = tarball.package_dir(tempdir.path())?;
    let script_path = package_dir.join("uninstall.sh").to_str()
        .map(|p| p.to_string())
        .ok_or(einval!("Cowardly refusing to execute non-UTF-8 path"))?;
    
    // Run install.sh
    let mut script = ExecBuilder::with_shell(&script_path)?;
    script.set_pwd(&package_dir);
    let process = run_verbose! {
        script.spawn_transparent()?,
        eprintln!("Running {:?}...", script_path)
    };
    process.wait()?;
    
    // Remove the archive
    fs::remove_file(archive_path)?;
    Ok(())
}
