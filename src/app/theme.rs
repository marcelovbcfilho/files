use iced::{Border, Color};
use iced::border::Radius;

pub struct ApplicationStyle;

impl ApplicationStyle {
    pub fn style(&self) -> iced::widget::container::Style {
        iced::widget::container::Style {
            border: Border{
                radius: Radius::new(16),
                ..Border::default()
            },
            background: Some(iced::Background::Color(Color::from_rgb(0.1, 0.2, 0.3))), // Custom background
            ..Default::default()
        }
    }
}