use ggez::graphics::Color;

pub const BUTTON_SIZE: (f32, f32) = (400.0, 100.0);
pub const SCREEN_SIZE: (f32, f32) = (1.3 * BUTTON_SIZE.0, 6.0 * BUTTON_SIZE.1);
pub const SETTINGS_SCREEN_SIZE: (f32, f32) = (1.3 * BUTTON_SIZE.0, 4.0 * BUTTON_SIZE.1);
pub const SCORES_SCREEN_SIZE: (f32, f32) = (15.0 * BUTTON_SIZE.1, 5.0 * BUTTON_SIZE.1);
pub const QUAD_SIZE: (f32, f32) = (BUTTON_SIZE.1 * 0.5, BUTTON_SIZE.1 * 0.5);
// pub const TOP_MARGIN: f32 = 0.1 * SCREEN_SIZE.1;

pub const BLUE: Color = Color {
    r: 0.19,
    g: 0.36,
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
    r: 0.25,
    g: 0.18,
    b: 0.74,
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
    r: 0.08,
    g: 0.08,
    b: 0.08,
    a: 1.0,
};
pub const BUTTON_HOVERED_COLOR: Color = Color {
    r: 0.2,
    g: 0.2,
    b: 0.2,
    a: 1.0,
};
pub const BUTTON_CLICKED_COLOR: Color = Color {
    r: 0.28,
    g: 0.28,
    b: 0.28,
    a: 1.0,
};

pub const BUTTON_CLEARED_COLOR: Color = Color {
    r: 0.18,
    g: 0.15,
    b: 0.14,
    a: 1.0,
};
pub const BUTTON_CLEARED_HOVERED_COLOR: Color = Color {
    r: 0.32,
    g: 0.30,
    b: 0.28,
    a: 1.0,
};
pub const BUTTON_CLEARED_CLICKED_COLOR: Color = Color {
    r: 0.48,
    g: 0.45,
    b: 0.42,
    a: 1.0,
};

pub const BUTTON_TEXT_COLOR: Color = Color {
    r: 0.85,
    g: 0.85,
    b: 0.85,
    a: 1.0,
};

pub const FLAG_COLOR: Color = Color {
    r: 0.85,
    g: 0.25,
    b: 0.25,
    a: 1.0,
};

pub const QUESTION_MARK_COLOR: Color = Color {
    r: 0.3,
    g: 0.8,
    b: 0.5,
    a: 1.0,
};

pub const MINE_COLOR: Color = Color {
    r: 0.66,
    g: 0.72,
    b: 0.84,
    a: 1.0,
};
