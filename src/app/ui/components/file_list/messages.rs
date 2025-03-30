use crate::app::ui::components::file_system_entry::messages::Message as FileSystemEntryMessage;

#[derive(Debug, Clone)]
pub enum Message {
    ResizeMessage(f32),
    FileSystemEntryMessage(FileSystemEntryMessage),
}
