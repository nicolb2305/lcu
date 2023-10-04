#![allow(dead_code)]
use crate::theme::Theme;

pub(crate) type Renderer = iced::Renderer<Theme>;
pub(crate) type Element<'a, Message> = iced::Element<'a, Message, Renderer>;
pub(crate) type Container<'a, Message> = iced::widget::Container<'a, Message, Renderer>;
pub(crate) type Button<'a, Message> = iced::widget::Button<'a, Message, Renderer>;
pub(crate) type Checkbox<'a, Message> = iced::widget::Checkbox<'a, Message, Renderer>;
pub(crate) type Text<'a> = iced::widget::Text<'a, Renderer>;
pub(crate) type Column<'a, Message> = iced::widget::Column<'a, Message, Renderer>;
pub(crate) type Row<'a, Message> = iced::widget::Row<'a, Message, Renderer>;
pub(crate) type Scrollable<'a, Message> = iced::widget::Scrollable<'a, Message, Renderer>;
