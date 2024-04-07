use std::{fs, io, path::Path};

use indicatif::ProgressBar;

pub fn copy_dir(source: &Path, destination: &Path, progress_bar: &ProgressBar) -> io::Result<()> {
    if !destination.exists() {
        fs::create_dir_all(destination)?;
    }

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let source_path = entry.path();
        let dest_path = destination.join(entry.file_name());

        if file_type.is_dir() {
            progress_bar.inc(1);
            copy_dir(&source_path, &dest_path, progress_bar)?;
        } else {
            progress_bar.inc(1);
            fs::copy(&source_path, &dest_path)?;
        }
    }

    Ok(())
}
