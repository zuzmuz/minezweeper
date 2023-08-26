mod grid;
use ggez::{
    graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect},
    Context, GameResult,
};
use grid::Grid;

pub struct Game {
    grid: Grid,
}

impl Game {

    pub fn new(shape: (usize, usize), number_of_mines: usize) -> Self {
        Game {
            grid: Grid::new(shape, number_of_mines)
        }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let (grid_x, grid_y) = self.grid.get_shape();
        for x in 0..grid_x {
            for y in 0..grid_y {
                let cell = self.grid.get(x, y);
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
                let rect = Rect::new(x as f32 * 50.0, y as f32 * 50.0, 50.0, 50.0);
                let rectangle =
                    Mesh::new_rounded_rectangle(ctx, DrawMode::fill(), rect, 5.0, color)?;
                canvas.draw(&rectangle, DrawParam::default())
            }
        }
        Ok(())
    }
}
