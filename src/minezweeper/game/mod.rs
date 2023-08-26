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
    last_hovered_cell: Option<(usize, usize)>,
}

impl Game {

    fn cell_point(x: usize, y: usize) -> Point2<f32> {
        Point2 {
            x: x as f32 * consts::QUAD_SIZE.0 + 0.1 * consts::QUAD_SIZE.0,
            y: y as f32 * consts::QUAD_SIZE.1 + 0.1 * consts::QUAD_SIZE.1,
        }
    }

    fn cell_size() -> (f32, f32) {
        (
            consts::QUAD_SIZE.0 - 0.2 * consts::QUAD_SIZE.0,
            consts::QUAD_SIZE.1 - 0.2 * consts::QUAD_SIZE.1,
        )
    }

    fn cell_rect(x: usize, y: usize) -> Rect {
        Rect::new(
            x as f32 * consts::QUAD_SIZE.0 + 0.1 * consts::QUAD_SIZE.0,
            y as f32 * consts::QUAD_SIZE.1 + 0.1 * consts::QUAD_SIZE.1,
            consts::QUAD_SIZE.0 - 0.2 * consts::QUAD_SIZE.0,
            consts::QUAD_SIZE.1 - 0.2 * consts::QUAD_SIZE.1,
        )
    }

    fn cell_position(x_pos: f32, y_pos: f32) -> (usize, usize) {
        (
            (x_pos / consts::QUAD_SIZE.0) as usize,
            (y_pos / consts::QUAD_SIZE.1) as usize,
        )
    }

    pub fn new(shape: (usize, usize), number_of_mines: usize) -> Self {
        Game {
            grid: Grid::new(shape, number_of_mines),
            last_hovered_cell: None,
        }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let (grid_x, grid_y) = self.grid.get_shape();
        for x in 0..grid_x {
            for y in 0..grid_y {
                let cell = self.grid.get(x, y);
                let rect = Self::cell_rect(x, y);
                let button_color = if cell.clicked {
                    consts::BUTTON_CLICKED_COLOR
                } else if cell.hovered {
                    consts::BUTTON_HOVERED_COLOR
                } else {
                    consts::BUTTON_COLOR
                };
                let rectangle = Mesh::new_rounded_rectangle(
                    ctx,
                    DrawMode::fill(),
                    rect,
                    0.2 * consts::QUAD_SIZE.0,
                    button_color,
                )?;
                canvas.draw(&rectangle, DrawParam::default());

                let value = cell.get_value();
                if value > 0 {
                    let color = consts::NUMBER_COLORS[(value - 1) as usize];
                    let text = Text::new(
                        TextFragment::new(value.to_string())
                            .scale(PxScale::from(0.8 * consts::QUAD_SIZE.1))
                            .font("SyneMono"),
                    );

                    let text_param = DrawParam::default()
                        .dest(Point2 {
                            x: rect.left() + 0.22 * consts::QUAD_SIZE.1,
                            y: rect.top() + 0.03 * consts::QUAD_SIZE.1,
                        })
                        .color(color);
                    canvas.draw(&text, text_param);
                }
            }
        }
        Ok(())
    }

    pub fn mouse_motion_event(
        &mut self,
        x_pos: f32,
        y_pos: f32,
    ) {
        let (cell_x, cell_y) = Self::cell_position(x_pos, y_pos);
        self.grid.set_hovered(cell_x, cell_y, true);
        if let Some((last_cell_x, last_cell_y)) = self.last_hovered_cell {
            if last_cell_x != cell_x || last_cell_y != cell_y {
                self.grid.set_hovered(last_cell_x, last_cell_y, false);
                self.grid.set_clicked(last_cell_x, last_cell_y, false);
                self.last_hovered_cell = Some((cell_x, cell_y));
            }
        }
        else {
            self.last_hovered_cell = Some((cell_x, cell_y));
        }
    }

    pub fn mouse_button_down_event(
        &mut self,
        x_pos: f32,
        y_pos: f32,
    ) {

        let (cell_x, cell_y) = Self::cell_position(x_pos, y_pos);
        self.grid.set_clicked(cell_x, cell_y, true);
    }

    pub fn mouse_button_up_event(
        &mut self,
        x_pos: f32,
        y_pos: f32,
    ) {
        let (grid_x, grid_y) = self.grid.get_shape();
        for x in 0..grid_x {
            for y in 0..grid_y {
                let rect = Self::cell_rect(x, y);
            }
        }
    }

    pub fn mouse_enter_or_leave(
        &mut self,
        entered: bool,
    ) {
        if !entered {
            if let Some((last_cell_x, last_cell_y)) = self.last_hovered_cell {
                self.grid.set_hovered(last_cell_x, last_cell_y, false);
                self.grid.set_clicked(last_cell_x, last_cell_y, false);
                self.last_hovered_cell = None;
            }
        }
    }
}
