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
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = Minezweeper::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct Minezweeper {
    grid: Grid,
}

impl Minezweeper {
    pub fn new(_ctx: &mut Context) -> Minezweeper {
        // Load/create resources such as images here.
        Minezweeper {
            grid: Grid::new((10, 10)),
        }
    }
}

impl event::EventHandler for Minezweeper {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
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
