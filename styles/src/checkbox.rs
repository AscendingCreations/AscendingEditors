use crate::TEXT_WHITE;
use iced::{widget::checkbox, Background, Color, Theme};
pub struct CheckBoxStyle;

impl checkbox::StyleSheet for CheckBoxStyle {
    type Style = Theme;

    fn active(&self, style: &Self::Style, _is_checked: bool) -> checkbox::Appearance {
        checkbox::Appearance {
            background: Background::Color(Color::from_rgb(0.95, 0.95, 0.95)),
            icon_color: Color::from_rgb(0.3, 0.3, 0.3),
            border_radius: 5.0.into(),
            border_width: 1.0,
            border_color: Color::from_rgb(0.6, 0.6, 0.6),
            text_color: Some(TEXT_WHITE),
        }
    }

    fn hovered(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        checkbox::Appearance {
            background: Background::Color(Color::from_rgb(0.90, 0.90, 0.90)),
            ..self.active(style, is_checked)
        }
    }
}
