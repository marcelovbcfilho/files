use crate::app::ui::components::file_system_entry::helper::{determine_entry_type, determine_icon};
use std::path::PathBuf;

#[derive(Clone)]
pub enum EntryType {
    File,
    Directory,
}

#[derive(Clone)]
pub enum IconType {
    File,
    Directory,
}

#[derive(Clone)]
pub struct FileSystemEntryModel {
    pub path: PathBuf,
    pub name: String,
    pub entry_type: EntryType,
    pub size: u64,
    pub icon: IconType,
}

impl FileSystemEntryModel {
    pub fn from_path(path: PathBuf) -> std::io::Result<Self> {
        let metadata = std::fs::metadata(&path)?;
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();
        let entry_type = determine_entry_type(&metadata);
        let icon = determine_icon(&entry_type);

        Ok(Self {
            path,
            name,
            entry_type,
            size: metadata.len(),
            icon,
        })
    }
}
