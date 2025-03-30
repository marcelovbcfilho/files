use crate::app::ui::components::file_list::data::FileListModel;
use crate::app::ui::components::file_list::messages::Message;
use crate::app::ui::components::file_system_entry::view::FileSystemEntryView;
use iced::widget::{Column, Container, Row, column, container, scrollable};
use iced::{Element, Length};

pub struct FileListView {
    model: FileListModel,
    column_count: usize,
    file_system_entry_views: Vec<FileSystemEntryView>,
}

impl FileListView {
    pub fn new(model: FileListModel) -> Self {
        let file_system_entry_views = model
            .file_entries
            .iter()
            .map(|entry| FileSystemEntryView::new(entry.clone()))
            .collect();

        Self {
            model,
            column_count: 8,
            file_system_entry_views,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ResizeMessage(width) => {
                self.column_count = ((width + 10.0) / (128.0 + 10.0)).floor() as usize
            }
            Message::FileSystemEntryMessage(_) => {}
        }
    }

    pub fn view(&self) -> Element<Message> {
        let mut row = Row::new().spacing(10);

        for (_, view) in self.file_system_entry_views.iter().enumerate() {
            row = row.push(view.view().map(Message::FileSystemEntryMessage));
        }
        

        scrollable(container(row.wrap()).width(Length::Fill))
            .height(Length::Fill)
            .into()
    }
}
