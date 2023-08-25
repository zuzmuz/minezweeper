use crate::consts;
mod grid;
mod menu;

use ggez::graphics::{self, Color};
use ggez::input::{
    keyboard::{KeyCode, KeyInput},
    mouse::MouseButton,
};
use ggez::{event::EventHandler, Context, GameResult};

use grid::Grid;
use menu::buttons::Button;

pub struct Minezweeper {
    grid: Option<Grid>,
    menu: [Button; 3],
}

impl Minezweeper {
    pub fn new(ctx: &mut Context) -> Minezweeper {
        // Load/create resources such as images here.
        let (frame_width, frame_height) = ctx.gfx.drawable_size();
        let (button_width, button_height) = consts::BUTTON_SIZE;
        let horizontal_margin = 0.5 * (frame_width - button_width);
        let vertical_margin = 0.25 * (frame_height - 3.0 * button_height);

        Minezweeper {
            grid: None,
            menu: [
                Button::new(
                    "Easy",
                    graphics::Rect::new(
                        horizontal_margin,
                        vertical_margin,
                        button_width,
                        button_height,
                    ),
                ),
                Button::new(
                    "Medium",
                    graphics::Rect::new(
                        horizontal_margin,
                        2.0 * vertical_margin + button_height,
                        button_width,
                        button_height,
                    ),
                ),
                Button::new(
                    "Hard",
                    graphics::Rect::new(
                        horizontal_margin,
                        3.0 * vertical_margin + 2.0 * button_height,
                        button_width,
                        button_height,
                    ),
                ),
            ],
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

    fn draw_menu(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        for button in &self.menu {
            button.draw(ctx, canvas, graphics::DrawParam::default())?;
        }
        Ok(())
    }
}

impl EventHandler for Minezweeper {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        if let Some(grid) = &self.grid {
            self.draw_grid(ctx, &mut canvas, &grid)?;
        } else {
            self.draw_menu(ctx, &mut canvas)?;
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
        println!("Mouse button pressed: {:?}, x: {}, y: {}", button, x, y);
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        println!("Mouse button released: {:?}, x: {}, y: {}", button, x, y);
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        println!("Key pressed: {:?}, repeat: {}", input, _repeat);
        Ok(())
    }
}
