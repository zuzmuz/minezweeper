use crate::consts;
use ggez::event::EventHandler;
use ggez::graphics::{self, Color};
use ggez::input::{
    keyboard::{KeyCode, KeyInput},
    mouse::MouseButton,
};
use ggez::{Context, GameResult};

use super::game::Game;
use super::menu::Menu;

enum Screen {
    Menu(Menu),
    Game(Game),
}

pub enum Level {
    Easy,
    Medium,
    Hard,
}

pub struct LevelInfo {
    pub name: String,
    pub grid_size: (usize, usize),
    pub number_of_mines: usize,
}

impl Level {
    pub fn level_info(&self) -> LevelInfo {
        match self {
            Self::Easy => LevelInfo {
                name: "Easy".to_string(),
                grid_size: (9, 9),
                number_of_mines: 10,
            },
            Self::Medium => LevelInfo {
                name: "Medium".to_string(),
                grid_size: (16, 16),
                number_of_mines: 40,
            },
            Self::Hard => LevelInfo {
                name: "Hard".to_string(),
                grid_size: (30, 16),
                number_of_mines: 250,
            },
        }
    }
}

pub struct Minezweeper {
    screen: Screen,
}

impl Minezweeper {
    pub fn new(_ctx: &mut Context) -> Minezweeper {
        // Load/create resources such as images here.
        Minezweeper {
            screen: Screen::Menu(Menu::standard()),
        }
    }
}

impl EventHandler for Minezweeper {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        match &self.screen {
            Screen::Menu(menu) => {
                menu.draw(ctx, &mut canvas)?;
            }
            Screen::Game(game) => {
                game.draw(ctx, &mut canvas)?;
            }
        }
        canvas.finish(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        match &mut self.screen {
            Screen::Menu(menu) => {
                menu.mouse_button_down_event(x, y);
            }
            Screen::Game(_grid) => {}
        }
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        match &mut self.screen {
            Screen::Menu(menu) => {
                if let Some(level) = menu.mouse_button_up_event(x, y) {
                    let level_info = level.level_info();
                    let grid_size = level_info.grid_size;
                    ctx.gfx.set_drawable_size(
                        grid_size.0 as f32 * consts::QUAD_SIZE.0,
                        grid_size.1 as f32 * consts::QUAD_SIZE.1,
                    )?;
                    self.screen = Screen::Game(Game::new(grid_size, level_info.number_of_mines))
                }
            }
            Screen::Game(_grid) => {}
        }
        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        x: f32,
        y: f32,
        _dx: f32,
        _dy: f32,
    ) -> GameResult {
        match &mut self.screen {
            Screen::Menu(menu) => {
                menu.mouse_motion_event(x, y);
            }
            Screen::Game(_grid) => {}
        }
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        if let Screen::Game(_grid) = &self.screen {
            if let Some(KeyCode::Back) = input.keycode {
                ctx.gfx.set_drawable_size(consts::SCREEN_SIZE.0, consts::SCREEN_SIZE.1)?;
                self.screen = Screen::Menu(Menu::standard())
            }
        }
        if let Some(KeyCode::Escape) = input.keycode {
            ctx.request_quit();
        }
        Ok(())
    }
}
