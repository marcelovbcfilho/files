use crate::app::ui::components::file_system_entry::data::FileSystemEntryModel;
use crate::app::ui::components::file_system_entry::helper::get_icon_path;
use crate::app::ui::components::file_system_entry::messages::Message;
use iced::widget::{column, container, text, svg, Svg};
use iced::{Element, alignment};

#[derive(Clone)]
pub struct FileSystemEntryView {
    model: FileSystemEntryModel,
}

impl FileSystemEntryView {
    pub fn new(model: FileSystemEntryModel) -> Self {
        Self { model }
    }

    pub fn update(&mut self, message: Message) {}

    pub fn view(&self) -> Element<Message> {
        let icon: Svg = svg(get_icon_path(&self.model.icon)).height(96).width(112);

        print!("{}", get_icon_path(&self.model.icon));

        container(column![icon, text(&self.model.name)].align_x(alignment::Horizontal::Center))
            .width(112)
            .height(128)
            .into()
    }
}
