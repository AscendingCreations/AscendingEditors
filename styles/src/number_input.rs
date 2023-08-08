use iced::{theme, widget::text_input, Background, Color, Theme};
use iced_aw::number_input;

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
pub struct CustomNumInput;
impl number_input::StyleSheet for CustomNumInput {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> number_input::Appearance {
        number_input::Appearance {
            icon_color: PRIMARY,
            ..number_input::Appearance::default()
        }
    }

    fn pressed(&self, style: &Self::Style) -> number_input::Appearance {
        self.active(style)
    }

    /// The appearance when the [`NumberInput`](crate::native::number_input::NumberInput) is disabled.
    fn disabled(&self, style: &Self::Style) -> number_input::Appearance {
        let active = self.active(style);
        number_input::Appearance {
            button_background: active.button_background.map(|bg| match bg {
                Background::Color(color) => Background::Color(Color {
                    a: color.a * 0.5,
                    ..color
                }),
                Background::Gradient(grad) => Background::Gradient(grad),
            }),
            icon_color: Color {
                a: active.icon_color.a * 0.5,
                ..active.icon_color
            },
        }
    }
}

pub struct CustomTextInput;
impl text_input::StyleSheet for CustomTextInput {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: BACKGROUND.into(),
            border_color: PRIMARY,
            border_width: 1.0,
            border_radius: 5.5.into(),
            icon_color: theme::Palette::DARK.text,
        }
    }

    fn hovered(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: BACKGROUND.into(),
            border_radius: 5.5.into(),
            border_width: 1.0,
            border_color: HOVERED,
            icon_color: theme::Palette::DARK.text,
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        let active = self.active(style);

        text_input::Appearance {
            border_color: PRIMARY,
            ..active
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        HOVERED
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        HOVERED
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        Color::WHITE
    }

    fn disabled(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: PRIMARY.into(),
            border_radius: 5.5.into(),
            border_width: 1.0,
            border_color: theme::Palette::DARK.text,
            icon_color: theme::Palette::DARK.text,
        }
    }

    fn disabled_color(&self, style: &Self::Style) -> Color {
        self.placeholder_color(style)
    }
}
