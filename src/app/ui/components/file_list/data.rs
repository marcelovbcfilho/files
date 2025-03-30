use crate::app::ui;
use crate::app::ui::components::file_system_entry::data::{EntryType, FileSystemEntryModel};
use std::fs;
use std::path::PathBuf;

#[derive(Clone)]
pub struct FileListModel {
    pub path: PathBuf,
    pub name: String,
    pub file_entries: Vec<FileSystemEntryModel>,
}

impl FileListModel {
    pub fn from_path(path: PathBuf) -> std::io::Result<Self> {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();
        let mut file_entries = Vec::new();

        if path.is_dir() {
            for entry in fs::read_dir(&path)? {
                let entry = entry?;
                let entry_path = entry.path();

                if let Ok(file_entry) = FileSystemEntryModel::from_path(entry_path) {
                    file_entries.push(file_entry);
                }
            }
        }

        file_entries.sort_by(|a, b| {
            match (&a.entry_type, &b.entry_type) {
                (EntryType::Directory, EntryType::File) => std::cmp::Ordering::Less,
                (EntryType::File, EntryType::Directory) => std::cmp::Ordering::Greater,
                _ => {
                    let a_is_dotfile = a.name.starts_with('.');
                    let b_is_dotfile = b.name.starts_with('.');

                    match (a_is_dotfile, b_is_dotfile) {
                        (true, true) | (false, false) => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                        (true, false) => std::cmp::Ordering::Greater,
                        (false, true) => std::cmp::Ordering::Less,
                    }
                }
            }
        });

        Ok(Self {
            path,
            name,
            file_entries,
        })
    }
}
