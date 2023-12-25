use chrono::{DateTime, Local};
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileInfoType {
    Directory,
    File,
    Symlink,
    // Add more types if needed
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileInfo {
    pub name: String,
    pub modified_time: DateTime<Local>,
    pub file_type: FileInfoType,
    pub size: u64,
}

pub fn get_file_info(
    show_hidden: bool,
    sort_by_size: bool,
    sort_by_time: bool,
) -> io::Result<Vec<FileInfo>> {
    let current_dir = std::env::current_dir()?;
    let entries = fs::read_dir(current_dir)?;

    let mut file_info_list = vec![];

    for entry in entries.filter_map(Result::ok) {
        let file_name = entry
            .file_name()
            .into_string()
            .unwrap_or_else(|_| String::from("Invalid Name"));
        let metadata = entry.metadata()?;
        let file_type = if metadata.is_dir() {
            FileInfoType::Directory
        } else if metadata.file_type().is_symlink() {
            FileInfoType::Symlink
        } else {
            FileInfoType::File
        };

        let modified_time = DateTime::from(metadata.modified()?);
        let size = if metadata.is_dir() {
            calculate_directory_size(&entry.path())?
        } else {
            metadata.len()
        };

        // Skip hidden files if flag not provided
        if !show_hidden && file_name.starts_with('.') {
            continue;
        }

        file_info_list.push(FileInfo {
            name: file_name,
            modified_time,
            file_type,
            size,
        });
    }

    if sort_by_size {
        file_info_list.sort_by(|a, b| a.size.cmp(&b.size));
    } else if sort_by_time {
        file_info_list.sort_by(|a, b| a.modified_time.cmp(&b.modified_time));
    } else {
        file_info_list.sort();
    }

    Ok(file_info_list)
}

fn calculate_directory_size(dir_path: &Path) -> io::Result<u64> {
    let mut total_size = 0;
    let entries = fs::read_dir(dir_path)?;

    for entry in entries.filter_map(Result::ok) {
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            total_size += calculate_directory_size(&entry.path())?;
        } else {
            total_size += metadata.len();
        }
    }

    Ok(total_size)
}
