#![allow(dead_code)]
use crate::theme::Theme;

pub type Renderer = iced::Renderer<Theme>;
pub type Element<'a, Message> = iced::Element<'a, Message, Renderer>;
pub type Container<'a, Message> = iced::widget::Container<'a, Message, Renderer>;
pub type Button<'a, Message> = iced::widget::Button<'a, Message, Renderer>;
pub type Checkbox<'a, Message> = iced::widget::Checkbox<'a, Message, Renderer>;
pub type Text<'a> = iced::widget::Text<'a, Renderer>;
pub type Column<'a, Message> = iced::widget::Column<'a, Message, Renderer>;
pub type Row<'a, Message> = iced::widget::Row<'a, Message, Renderer>;
pub type Scrollable<'a, Message> = iced::widget::Scrollable<'a, Message, Renderer>;
pub type Slider<'a, Message> = iced::widget::Slider<'a, Message, Renderer>;
