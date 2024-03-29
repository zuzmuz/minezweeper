mod consts;
mod minezweeper;
use ggez::{event, graphics::FontData, ContextBuilder};
use minezweeper::Minezweeper;
use std::env;
use std::path;

fn main() {
    // fetching resource directory
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (mut ctx, event_loop) = ContextBuilder::new("minezweeper", "zzz")
        .window_setup(ggez::conf::WindowSetup::default().title("minezweeper"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(consts::SCREEN_SIZE.0, consts::SCREEN_SIZE.1)
        )
        .add_resource_path(resource_dir)
        .build()
        .expect("aieee, could not create ggez context!");

    let font = FontData::from_path(&ctx.fs, "/Synemono-Regular.ttf").expect("Could not load font");
    ctx.gfx.add_font("SyneMono", font);
    let minezweeper = Minezweeper::new(&mut ctx);

    event::run(ctx, event_loop, minezweeper);
}
