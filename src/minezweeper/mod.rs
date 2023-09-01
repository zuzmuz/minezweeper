mod game;
mod menu;
mod settings;

use crate::consts;
use crate::minezweeper::{
    game::{Game, GameState},
    menu::Menu, menu::Selected, menu::settings::Settings,
    settings::Controls,
    settings::Score,
};
use ggez::event::EventHandler;
use ggez::graphics::{self, Color};
use ggez::input::{
    keyboard::{KeyCode, KeyInput},
    mouse::MouseButton,
};
use ggez::{Context, GameResult};

enum Screen {
    Menu(Menu),
    Game(Game),
    Settings(Settings),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
                number_of_mines: 99,
            },
        }
    }
}

pub struct Minezweeper {
    screen: Screen,
    controls: Controls,
    started_level: Option<Level>,
}

impl Minezweeper {
    pub fn new(_ctx: &mut Context) -> Minezweeper {
        // Load/create resources such as images here.
        Minezweeper {
            screen: Screen::Menu(Menu::standard()),
            controls: Controls::default(),
            started_level: None,
        }
    }

    fn start_game(&mut self, level: Level, ctx: &mut Context) -> GameResult {
        self.started_level = Some(level);
        let level_info = level.level_info();
        let grid_size = level_info.grid_size;
        ctx.gfx.set_drawable_size(
            grid_size.0 as f32 * consts::QUAD_SIZE.0,
            consts::QUAD_SIZE.1 + grid_size.1 as f32 * consts::QUAD_SIZE.1,
        )?;
        self.screen = Screen::Game(Game::new(grid_size, level_info.number_of_mines));
        Ok(())
    }

    fn end_game(&mut self, game_state: GameState) {
        if let Some(level) = self.started_level {
            if let Err(err) = Score::new(
                level,
                game_state,
                0.0,
            )
            .save()
            {
                println!("Error writing score to file: {}", err);
            }
            else {
                println!("Score written to file {:?}", game_state);
            }
            self.started_level = None;
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
            Screen::Settings(settings) => {
                settings.draw(ctx, &mut canvas)?;
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
            Screen::Game(game) => {
                game.mouse_button_down_event(x, y);
            }
            Screen::Settings(_) => {
                // settings.mouse_button_down_event(x, y);
            }
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
                match menu.mouse_button_up_event(x, y) {
                    Selected::Level(level) => self.start_game(level, ctx)?,
                    Selected::Settings => self.screen = Screen::Settings(Settings::standard()),
                    Selected::None => {}
                }
            }
            Screen::Game(game) => {
                let game_state = game.mouse_button_up_event(x, y);
                if game_state != GameState::Playing {
                    self.end_game(game_state)
                }
            }
            Screen::Settings(_) => {

            }
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
            Screen::Menu(menu) => menu.mouse_motion_event(x, y),
            Screen::Game(game) => game.mouse_motion_event(x, y),
            Screen::Settings(_) => {
                //settings.mouse_motion_event(x, y),
            }
        }
        Ok(())
    }

    fn mouse_enter_or_leave(&mut self, _ctx: &mut Context, entered: bool) -> GameResult {
        if let Screen::Game(game) = &mut self.screen {
            game.mouse_enter_or_leave(entered);
        }
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        match &mut self.screen {
            Screen::Menu(_) => {
                match input.keycode {
                    Some(KeyCode::Key1) => self.start_game(Level::Easy, ctx)?,
                    Some(KeyCode::Key2) => self.start_game(Level::Medium, ctx)?,
                    Some(KeyCode::Key3) => self.start_game(Level::Hard, ctx)?,
                    _ => {}
                };
            }
            Screen::Game(game) => match input.keycode {
                Some(KeyCode::Back) => {
                    ctx.gfx
                        .set_drawable_size(consts::SCREEN_SIZE.0, consts::SCREEN_SIZE.1)?;
                    self.end_game(GameState::Abandoned);
                    self.screen = Screen::Menu(Menu::standard())
                }
                Some(keycode) => {
                    let game_state = game.handle(self.controls.handle(keycode));
                    if game_state != GameState::Playing {
                        self.end_game(game_state)
                    }
                },
                None => {}
            }
            Screen::Settings(_) => match input.keycode {
                Some(KeyCode::Back) => {
                    ctx.gfx
                        .set_drawable_size(consts::SCREEN_SIZE.0, consts::SCREEN_SIZE.1)?;
                    self.end_game(GameState::Abandoned);
                    self.screen = Screen::Menu(Menu::standard())
                }
                Some(_) | None => {}
            }
        }
        if let Some(KeyCode::Escape) = input.keycode {
            ctx.request_quit();
        }
        Ok(())
    }
}
