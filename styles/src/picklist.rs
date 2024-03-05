use iced::{widget::pick_list, Background, Color, Theme, Border};

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

pub struct CustomPickList;
impl pick_list::StyleSheet for CustomPickList {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> pick_list::Appearance {
        pick_list::Appearance {
            text_color: Color::WHITE,
            background: Background::Color(BACKGROUND),
            border: Border {
                color: PRIMARY,
                width: 1.0,
                radius: 2.0.into(),
            },
            placeholder_color: PRIMARY,
            handle_color: PRIMARY,
        }
    }

    fn hovered(&self, style: &Self::Style) -> pick_list::Appearance {
        pick_list::Appearance {
            border: Border {
                color: HOVERED,
                ..self.active(style).border
            },
            ..self.active(style)
        }
    }
}
