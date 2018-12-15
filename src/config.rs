use std::ffi::OsStr;
use std::fs::{read_dir, DirEntry};
use std::io;

fn is_config_file(dir_entry: &DirEntry) -> bool {
    let path = dir_entry.path();
    let extension = path.extension();

    const CONFIG_FILE_EXTENSION: &str = "ini";
    extension == Some(&OsStr::new(CONFIG_FILE_EXTENSION))
}

pub(crate) fn get_config_files(
    config_path: &str,
) -> io::Result<impl Iterator<Item = io::Result<DirEntry>>> {
    let entries = read_dir(config_path)?;

    let filtered_entries = entries.filter(|result| match result {
        Err(_) => true,
        Ok(dir_entry) => is_config_file(dir_entry),
    });

    Ok(filtered_entries)
}
