use crate::app::ui::components::file_list::data::FileListModel;
use crate::app::ui::components::file_list::messages::Message as FileListMessage;
use crate::app::ui::components::file_list::view::FileListView;
use iced::widget::container::background;
use iced::widget::pane_grid::Pane;
use iced::widget::{PaneGrid, column, container, pane_grid, text};
use iced::{Color, Element, Length, Size, Theme, application, window};
use pane_grid::Content;
use std::path::PathBuf;

mod app;

pub fn main() {
    application(FilesApp::title, FilesApp::update, FilesApp::view)
        .theme(move |_x| Theme::Ferra)
        .centered()
        .window(window::Settings {
            size: Size::new(1280.0, 720.0),
            decorations: false,
            transparent: true,
            ..Default::default()
        })
        .run()
        .unwrap();
}

pub struct FilesApp {
    panes: pane_grid::State<PaneContent>,
    file_list_model: FileListModel,
    file_list_view: FileListView,
}

impl Default for FilesApp {
    fn default() -> Self {
        let pane1 = PaneContent {
            title: "Left Pane".to_string(),
            position: PanePosition::Left,
        };
        let pane2 = PaneContent {
            title: "Right Pane".to_string(),
            position: PanePosition::Right,
        };
        let (mut panes, pane) = pane_grid::State::new(pane1);
        let (_, split) = panes.split(pane_grid::Axis::Vertical, pane, pane2).unwrap();
        panes.resize(split, 0.15);

        let initial_folder = PathBuf::from("/Users/marcelovbcfilho");
        let file_list_model = FileListModel::from_path(initial_folder).unwrap();
        let file_list_view = FileListView::new(file_list_model.clone());

        Self {
            panes,
            file_list_model,
            file_list_view,
        }
    }
}

pub struct PaneContent {
    title: String,
    position: PanePosition,
}

pub enum PanePosition {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub enum Message {
    PaneResized(pane_grid::ResizeEvent),
    FileListMessage(FileListMessage),
}

impl FilesApp {
    fn title(&self) -> String {
        String::from("Test")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::PaneResized(resize_event) => {
                self.panes.resize(resize_event.split, resize_event.ratio);
            }
            _ => {}
        }
    }

    fn view(&self) -> Element<Message> {
        let content = |_pane: Pane, pane_content: &PaneContent, _maximized: bool| {
            let bg_color = match pane_content.position {
                PanePosition::Left => Color::from_rgb(0.2, 0.2, 0.8), // Blue for left
                PanePosition::Right => Color::from_rgb(0.8, 0.2, 0.2), // Red for right
            };

            match pane_content.position {
                PanePosition::Left => Content::new(
                    container(column![text(pane_content.title.to_owned())].height(Length::Fill))
                        .height(Length::Fill)
                        .style(move |_x| background(bg_color)),
                ),
                PanePosition::Right => Content::new(
                    container(self.file_list_view.view().map(Message::FileListMessage))
                        .style(move |_x| background(bg_color)),
                ),
            }
        };

        container(pane_grid(&self.panes, content).on_resize(10, Message::PaneResized))
            .height(Length::Fill)
            .width(Length::Fill)
            .style(|_| app::theme::ApplicationStyle.style())
            .into()
    }
}
