#![allow(dead_code, unused_variables)]
use iced::{
    application, color,
    widget::{button, checkbox, container, text},
    Background, BorderRadius,
};

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Theme;

#[derive(Debug, Clone, Copy, Default)]
pub(crate) enum Container {
    #[default]
    Default,
    Bordered,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) enum Button {
    #[default]
    Primary,
    Secondary,
}

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: color!(0x01, 0x0A, 0x13),
            text_color: color!(0xB6, 0x99, 0x5F),
        }
    }
}

impl text::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: Self::Style) -> text::Appearance {
        text::Appearance {
            color: color!(0xB6, 0x99, 0x5F).into(),
        }
    }
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            Container::Default => container::Appearance::default(),
            Container::Bordered => container::Appearance {
                border_color: color!(0xB5, 0x8B, 0x33),
                border_width: 1.0,
                // border_radius: 4.0,
                ..Default::default()
            },
        }
    }
}

impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Button::Primary => button::Appearance {
                background: Some(color!(0x0F, 0x18, 0x1E).into()),
                // border_radius: 4.0,
                border_width: 1.0,
                border_color: color!(0xB5, 0x8B, 0x33),
                ..Default::default()
            },
            Button::Secondary => button::Appearance {
                background: Some(color!(0x0F, 0x18, 0x1E).into()),
                ..Default::default()
            },
        }
    }
}

impl checkbox::StyleSheet for Theme {
    type Style = ();

    fn active(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        checkbox::Appearance {
            background: Background::Color(color!(0x01, 0x0C, 0x15)),
            icon_color: color!(0xBB, 0x91, 0x3A),
            border_radius: BorderRadius::from(1.0),
            border_width: 1.0,
            border_color: color!(0x5A, 0x46, 0x23),
            text_color: Some(color!(0xA0, 0x9B, 0x8C)),
        }
    }

    fn hovered(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        checkbox::Appearance {
            background: Background::Color(color!(0x01, 0x0C, 0x15)),
            icon_color: color!(0xBB, 0x91, 0x3A),
            border_radius: BorderRadius::from(1.0),
            border_width: 1.0,
            border_color: color!(0x75, 0x63, 0x3A),
            text_color: Some(color!(0xA0, 0x9B, 0x8C)),
        }
    }
}
