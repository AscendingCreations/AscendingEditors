use iced::{text_input, Color};
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
    fn active(&self) -> number_input::Style {
        number_input::Style {
            icon_color: PRIMARY,
            ..number_input::Style::default()
        }
    }
}

pub struct CustomTextInput;
impl text_input::StyleSheet for CustomTextInput {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: BACKGROUND.into(),
            border_color: PRIMARY,
            border_width: 1.0,
            border_radius: 5.5,
        }
    }

    fn focused(&self) -> text_input::Style {
        let active = self.active();

        text_input::Style {
            border_color: PRIMARY,
            ..active
        }
    }

    fn placeholder_color(&self) -> Color {
        HOVERED
    }

    fn selection_color(&self) -> Color {
        HOVERED
    }

    fn value_color(&self) -> Color {
        Color::WHITE
    }
}
