use ggez::graphics::Color;

pub const BUTTON_SIZE: (f32, f32) = (400.0, 100.0);
pub const QUAD_SIZE: (f32, f32) = (50.0, 50.0);

pub const BUTTON_COLOR: Color = Color {
    r: 0.1,
    g: 0.1,
    b: 0.1,
    a: 1.0,
};
pub const BUTTON_HOVERED_COLOR: Color = Color {
    r: 0.15,
    g: 0.15,
    b: 0.15,
    a: 1.0,
};
pub const BUTTON_CLICKED_COLOR: Color = Color {
    r: 0.5,
    g: 0.5,
    b: 0.5,
    a: 1.0,
};
pub const BUTTON_TEXT_COLOR: Color = Color {
    r: 0.85,
    g: 0.85,
    b: 0.85,
    a: 1.0,
};
