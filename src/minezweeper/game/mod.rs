mod grid;
use ggez::{
    graphics::{Canvas, DrawMode, DrawParam, Mesh, PxScale, Rect, Text, TextFragment},
    mint::Point2,
    Context, GameResult,
};
use grid::Grid;

use crate::consts;

pub struct Game {
    grid: Grid,
}

impl Game {
    pub fn new(shape: (usize, usize), number_of_mines: usize) -> Self {
        Game {
            grid: Grid::new(shape, number_of_mines),
        }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let (grid_x, grid_y) = self.grid.get_shape();
        for x in 0..grid_x {
            for y in 0..grid_y {
                let cell = self.grid.get(x, y);
                let rect = Rect::new(
                    x as f32 * consts::QUAD_SIZE.0 + 0.1 * consts::QUAD_SIZE.0,
                    y as f32 * consts::QUAD_SIZE.1 + 0.1 * consts::QUAD_SIZE.1,
                    consts::QUAD_SIZE.0 - 0.1 * consts::QUAD_SIZE.0,
                    consts::QUAD_SIZE.1 - 0.1 * consts::QUAD_SIZE.1,
                );
                let rectangle = Mesh::new_rounded_rectangle(
                    ctx,
                    DrawMode::fill(),
                    rect,
                    0.2 * consts::QUAD_SIZE.0,
                    consts::BUTTON_COLOR,
                )?;
                canvas.draw(&rectangle, DrawParam::default());

                if cell > 0 {
                    let color = consts::NUMBER_COLORS[(cell - 1) as usize];
                    let text = Text::new(
                        TextFragment::new(cell.to_string())
                            .scale(PxScale::from(0.8 * consts::QUAD_SIZE.1))
                            .font("SyneMono"),
                    );

                    let text_param = DrawParam::default()
                        .dest(Point2 {
                            x: rect.left() + 0.27 * consts::QUAD_SIZE.1,
                            y: rect.top() + 0.04 * consts::QUAD_SIZE.1,
                        })
                        .color(color);
                    canvas.draw(&text, text_param);
                }
            }
        }
        Ok(())
    }
}
