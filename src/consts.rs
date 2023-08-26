use ggez::graphics::Color;

pub const BUTTON_SIZE: (f32, f32) = (400.0, 100.0);
pub const SCREEN_SIZE: (f32, f32) = (1.5 * BUTTON_SIZE.0, 4.0 * BUTTON_SIZE.1);
pub const QUAD_SIZE: (f32, f32) = (50.0, 50.0);

pub const BLUE: Color = Color {
    r: 0.15,
    g: 0.31,
    b: 0.99,
    a: 1.0,
};

pub const GREEN: Color = Color {
    r: 0.22,
    g: 0.85,
    b: 0.30,
    a: 1.0,
};

pub const RED: Color = Color {
    r: 0.95,
    g: 0.32,
    b: 0.11,
    a: 1.0,
};

pub const DARK_BLUE: Color = Color {
    r: 0.20,
    g: 0.35,
    b: 0.72,
    a: 1.0,
};

pub const DARK_RED: Color = Color {
    r: 0.65,
    g: 0.32,
    b: 0.25,
    a: 1.0,
};

pub const TURQUOISE: Color = Color {
    r: 0.35,
    g: 0.78,
    b: 0.71,
    a: 1.0,
};

pub const PURPLE: Color = Color {
    r: 0.82,
    g: 0.25,
    b: 0.78,
    a: 1.0,
};

pub const GREEN_GRAY: Color = Color {
    r: 0.60,
    g: 0.74,
    b: 0.22,
    a: 1.0,
};

pub const NUMBER_COLORS: [Color; 8] = [
    BLUE, GREEN, RED, DARK_BLUE, DARK_RED, TURQUOISE, PURPLE, GREEN_GRAY,
];

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
