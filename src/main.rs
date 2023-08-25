mod grid;
use ggez::graphics::{self, Color};
use ggez::input::{
    keyboard::{KeyCode, KeyInput},
    mouse::MouseButton,
};
use ggez::{
    event::{self, EventHandler},
    Context, ContextBuilder, GameResult,
};
use grid::Grid;

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("minezweeper", "zzz")
        .window_setup(ggez::conf::WindowSetup::default().title("minezweeper"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(800.0, 600.0)
                .borderless(true)
                .resizable(true),
        )
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let minezweeper = Minezweeper::new(&mut ctx);

    event::run(ctx, event_loop, minezweeper);
}

struct Minezweeper {
    pub grid: Option<Grid>,
}

impl Minezweeper {
    pub fn new(ctx: &mut Context) -> Minezweeper {
        // Load/create resources such as images here.
        Minezweeper { grid: None }
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
        let rect = graphics::Rect::new(0.0, 0.0, 800.0, 600.0);
        let rectangle = graphics::Mesh::new_rounded_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect,
            5.0,
            Color::WHITE,
        )?;
        canvas.draw(&rectangle, graphics::DrawParam::default());
        Ok(())
    }
}

impl event::EventHandler for Minezweeper {
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
