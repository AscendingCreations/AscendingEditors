use crate::TEXT_WHITE;
use iced::{Background, Color};
use iced_style::checkbox::{Style, StyleSheet};
pub struct CheckBoxStyle;

impl StyleSheet for CheckBoxStyle {
    fn active(&self, _is_checked: bool) -> Style {
        Style {
            background: Background::Color(Color::from_rgb(0.95, 0.95, 0.95)),
            checkmark_color: Color::from_rgb(0.3, 0.3, 0.3),
            border_radius: 5.0,
            border_width: 1.0,
            border_color: Color::from_rgb(0.6, 0.6, 0.6),
            text_color: Some(TEXT_WHITE),
        }
    }

    fn hovered(&self, is_checked: bool) -> Style {
        Style {
            background: Background::Color(Color::from_rgb(0.90, 0.90, 0.90)),
            ..self.active(is_checked)
        }
    }
}
