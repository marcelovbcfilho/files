use crate::app::ui::components::file_system_entry::data::{EntryType, IconType};
use std::fs::Metadata;

pub fn determine_icon(entry_type: &EntryType) -> IconType {
    match entry_type {
        EntryType::Directory => IconType::Directory,
        EntryType::File => IconType::File,
    }
}

pub fn determine_entry_type(metadata: &Metadata) -> EntryType {
    if metadata.is_dir() {
        EntryType::Directory
    } else {
        EntryType::File
    }
}

pub fn get_icon_path(icon_type: &IconType) -> String {
    match icon_type {
        IconType::Directory => "src/assets/icons/folder_icon.svg".to_string(),
        IconType::File => "src/assets/icons/file_icon.svg".to_string(),
    }
}
