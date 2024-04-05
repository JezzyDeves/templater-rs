use std::{
    fs,
    io::{self, stdout},
    path::Path,
};

use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};

pub fn copy_dir(
    source: &Path,
    destination: &Path,
    number_of_completed_files: &mut i32,
    total_files: usize,
) -> io::Result<()> {
    if !destination.exists() {
        fs::create_dir_all(destination)?;
    }

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let source_path = entry.path();
        let dest_path = destination.join(entry.file_name());

        if file_type.is_dir() {
            update_progress(number_of_completed_files, total_files);
            copy_dir(
                &source_path,
                &dest_path,
                number_of_completed_files,
                total_files,
            )?;
        } else {
            update_progress(number_of_completed_files, total_files);
            fs::copy(&source_path, &dest_path)?;
        }
    }

    Ok(())
}

fn update_progress(number_of_completed_files: &mut i32, total_files: usize) {
    *number_of_completed_files += 1;
    execute!(stdout(), Clear(ClearType::All)).unwrap();
    println!("Progress: [{}/{}]", number_of_completed_files, total_files);
}
