mod minezweeper;
mod grid;
mod menu;
mod consts;
use ggez::{event, ContextBuilder, graphics::FontData};
use std::env;
use std::path;
use minezweeper::Minezweeper;

fn main() {

    // fetching resource directory
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("path11: {:?}", path);
        path
    } else {
        path::PathBuf::from("./resources")
    };


    let (button_width, button_height) = consts::BUTTON_SIZE;
    let (mut ctx, event_loop) = ContextBuilder::new("minezweeper", "zzz")
        .window_setup(ggez::conf::WindowSetup::default().title("minezweeper"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(1.5*button_width, 4.0 * button_height)
                .borderless(true),
        )
        .add_resource_path(resource_dir)
        .build()
        .expect("aieee, could not create ggez context!");


    let font = FontData::from_path(&ctx.fs, "/Synemono-Regular.ttf").expect("Could not load font");
    ctx.gfx.add_font("SyneMono", font);
    

    let minezweeper = Minezweeper::new(&mut ctx);

    event::run(ctx, event_loop, minezweeper);
}