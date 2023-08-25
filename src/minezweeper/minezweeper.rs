use std::default;

use ggez::graphics::{self, Color};
use ggez::input::{
    keyboard::{KeyCode, KeyInput},
    mouse::MouseButton,
};
use ggez::{event::EventHandler, Context, GameResult};

use super::grid::Grid;
use super::menu::{Menu, MenuDelegate};

enum Screen {
    Menu(Menu),
    Game(Grid),
}

pub enum Level {
    Easy,
    Medium,
    Hard
}

pub struct LevelInfo {
    pub name: String,
    pub grid_size: (usize, usize),
    pub number_of_mines: usize
}

impl Level {
    pub fn level_info(&self) -> LevelInfo {
        match self {
            Self::Easy => { 
                LevelInfo {
                    name: "Easy".to_string(),
                    grid_size: (9, 9),
                    number_of_mines: 10
                }
            }
            Self::Medium => { 
                LevelInfo {
                    name: "Medium".to_string(),
                    grid_size: (16, 16),
                    number_of_mines: 40
                }
            }
            Self::Hard => { 
                LevelInfo {
                    name: "Hard".to_string(),
                    grid_size: (16, 40),
                    number_of_mines: 99
                }
            }
        }
    }
}

pub trait MouseMineEventHandler {
    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32
    ) -> GameResult;

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult;

    fn mouse_motion_event(
        &mut self,
        ctx: &mut Context,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
    ) -> GameResult;
}

pub struct Minezweeper {
    screen: Screen,
}

impl Minezweeper {
    pub fn new(ctx: &mut Context) -> Minezweeper {
        // Load/create resources such as images here.
        let (frame_width, frame_height) = ctx.gfx.drawable_size();
        Minezweeper {
            screen: Screen::Menu(Menu::standard(frame_width, frame_height)),
        }
    }

    fn draw_grid(
        &self,
        ctx: &mut Context,
        canvas: &mut graphics::Canvas,
        grid: &Grid,
    ) -> GameResult {
        let (grid_x, grid_y) = grid.get_shape();
        for x in 0..grid_x {
            for y in 0..grid_y {
                let cell = grid.get(x, y);
                let color = match cell {
                    -1 => Color::RED,
                    0 => Color::WHITE,
                    1 => Color::GREEN,
                    2 => Color::BLUE,
                    3 => Color::YELLOW,
                    4 => Color::CYAN,
                    5 => Color::MAGENTA,
                    6 => Color::BLACK,
                    7 => Color::WHITE,
                    8 => Color::BLACK,
                    _ => Color::WHITE,
                };
                let rect = graphics::Rect::new(x as f32 * 50.0, y as f32 * 50.0, 50.0, 50.0);
                let rectangle = graphics::Mesh::new_rounded_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    rect,
                    5.0,
                    color,
                )?;
                canvas.draw(&rectangle, graphics::DrawParam::default())
            }
        }
        Ok(())
    }

    // fn draw_menu(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        
    // }
}

impl MenuDelegate for Minezweeper {
    fn level_selected(&mut self, level: Level) {
        self.screen = Screen::Game(
            Grid::new(level.level_info().grid_size, level.level_info().number_of_mines)
        )
    }
}

impl EventHandler for Minezweeper {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        match &self.screen {
            Screen::Menu(menu) => {
                menu.draw(ctx, &mut canvas)?;
            }
            Screen::Game(grid) => {
                self.draw_grid(ctx, &mut canvas, &grid)?;
            }
        }
        canvas.finish(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        match &mut self.screen {
            Screen::Menu(menu) => {
                menu.mouse_button_down_event(ctx, button, x, y)
            },
            Screen::Game(grid) => {
                Ok(())
            }
        }
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        match &mut self.screen {
            Screen::Menu(menu) => {
                menu.mouse_button_up_event(ctx, button, x, y)
            },
            Screen::Game(grid) => {
                Ok(())
            }
        }
    }

    fn mouse_motion_event(
        &mut self,
        ctx: &mut Context,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
    ) -> GameResult {
        match &mut self.screen {
            Screen::Menu(menu) => {
                menu.mouse_motion_event(ctx, x, y, dx, dy)
            },
            Screen::Game(grid) => {
                Ok(())
            }
        }
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        println!("Key pressed: {:?}, repeat: {}", input, _repeat);
        Ok(())
    }
}