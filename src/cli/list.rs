use crate::{ error::Result, packagedir::Packagedir };
use std::fs;


/// Lists all installed packages
pub fn exec() -> Result {
    // Walk the entries
    let packagedir = Packagedir::new()?;
    for entry in fs::read_dir(&packagedir)? {
        // Unwrap the entry
        let entry = entry?;
        let entry_path = entry.path();
        let entry_name = entry.file_name().to_string_lossy().to_string();

        // Check if the entry is probably a tar.gz archive
        if entry_path.is_file() && entry_name.ends_with(".tar.gz") {
            println!("{}", entry_name);
        }
    }
    Ok(())
}
