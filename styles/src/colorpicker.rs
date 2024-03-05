use iced::{Theme, Color};
use iced_aw::style::color_picker::{Appearance, StyleSheet};

const BACKGROUND: Color = Color {
    r: 48.0 / 255.0,
    g: 54.0 / 255.0,
    b: 61.0 / 255.0,
    a: 1.0,
};

const PRIMARY: Color = Color {
    r: 110.0 / 255.0,
    g: 118.0 / 255.0,
    b: 129.0 / 255.0,
    a: 1.0,
};

const HOVERED: Color = Color {
    r: 131.0 / 255.0,
    g: 139.0 / 255.0,
    b: 149.0 / 255.0,
    a: 1.0,
};

pub struct CustomColorPicker;

impl StyleSheet for CustomColorPicker {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            background: BACKGROUND.into(),
            border_radius: 15.0,
            border_width: 1.0,
            border_color: PRIMARY,
            bar_border_radius: 5.0,
            bar_border_width: 1.0,
            bar_border_color: PRIMARY,
        }
    }

    fn selected(&self, style: &Self::Style) -> Appearance {
        Appearance {
            ..self.active(style)
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        Appearance {
            ..self.active(style)
        }
    }

    fn focused(&self, style: &Self::Style) -> Appearance {
        Appearance {
            border_color: HOVERED,
            bar_border_color: HOVERED,
            ..self.active(style)
        }
    }
}
