mod grid;

use ggez::{
    graphics::{Canvas, DrawMode, DrawParam, Mesh, PxScale, Rect, Text, TextFragment},
    mint::Point2,
    Context, GameResult,
};
use grid::Grid;

use crate::{consts, settings::{Action, Direction}};

pub enum GameState {
    Win,
    Lose,
    Playing,
}
pub struct Game {
    grid: Grid,
    last_hovered_cell: Option<(usize, usize)>,
}

impl Game {
    fn cell_rect(x: usize, y: usize) -> Rect {
        Rect::new(
            x as f32 * consts::QUAD_SIZE.0 + 0.1 * consts::QUAD_SIZE.0,
            consts::QUAD_SIZE.1 + y as f32 * consts::QUAD_SIZE.1 + 0.1 * consts::QUAD_SIZE.1,
            consts::QUAD_SIZE.0 - 0.2 * consts::QUAD_SIZE.0,
            consts::QUAD_SIZE.1 - 0.2 * consts::QUAD_SIZE.1,
        )
    }

    fn cell_position(x_pos: f32, y_pos: f32) -> Option<(usize, usize)> {
        if y_pos - consts::QUAD_SIZE.1 < 0.0 {
            None
        }
        else {
            Some((
                (x_pos / consts::QUAD_SIZE.0) as usize,
                ((y_pos - consts::QUAD_SIZE.1)/ consts::QUAD_SIZE.1) as usize,
            ))
        }
    }

    pub fn new(shape: (usize, usize), number_of_mines: usize) -> Self {
        Game {
            grid: Grid::new(shape, number_of_mines),
            last_hovered_cell: None,
        }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {

        let color = consts::FLAG_COLOR;
        let text = Text::new(
            TextFragment::new(self.grid.get_number_of_remaining_mines().to_string())
                .scale(PxScale::from(0.9 * consts::QUAD_SIZE.1))
                .font("SyneMono"),
        );

        let text_param = DrawParam::default()
            .dest(Point2 {
                x: ((self.grid.get_shape().0 - 1) as f32) * consts::QUAD_SIZE.1,
                y: 0.1 * consts::QUAD_SIZE.1,
            })
            .color(color);
        canvas.draw(&text, text_param);


        let (grid_x, grid_y) = self.grid.get_shape();
        for x in 0..grid_x {
            for y in 0..grid_y {
                let cell = self.grid.get(x, y);
                let rect = Self::cell_rect(x, y);
                let button_color = match (cell.hovered, cell.clicked, cell.cleared) {
                    (true, true, true) => consts::BUTTON_CLEARED_CLICKED_COLOR,
                    (true, true, false) => consts::BUTTON_CLICKED_COLOR,
                    (true, false, true) => consts::BUTTON_CLEARED_HOVERED_COLOR,
                    (true, false, false) => consts::BUTTON_HOVERED_COLOR,
                    (false, _, true) => consts::BUTTON_CLEARED_COLOR,
                    (false, _, false) => consts::BUTTON_COLOR,
                };
                let rectangle = Mesh::new_rounded_rectangle(
                    ctx,
                    DrawMode::fill(),
                    rect,
                    0.2 * consts::QUAD_SIZE.0,
                    button_color,
                )?;
                canvas.draw(&rectangle, DrawParam::default());

                if cell.cleared {
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
                else if cell.flagged {
                    //Draw a flag
                    let flag = Mesh::new_rounded_rectangle(ctx, DrawMode::fill(), Rect::new(
                        rect.center().x - 0.025 * rect.w, rect.top() + 0.24 * rect.h,
                        0.05 * consts::QUAD_SIZE.0, 0.52 * rect.h, 
                    ), 0.1 * consts::QUAD_SIZE.0, consts::FLAG_COLOR)?;

                    canvas.draw(&flag, DrawParam::default());

                    let flag = Mesh::new_polygon(ctx, DrawMode::fill(), &[
                        Point2 { x: rect.center().x, y: rect.top() + 0.24 * rect.h },
                        Point2 { x: rect.center().x + 0.3 * rect.w, y: rect.top() + 0.37 * rect.h },
                        Point2 { x: rect.center().x, y: rect.top() + 0.5 * rect.h },
                    ], consts::FLAG_COLOR)?;
                    canvas.draw(&flag, DrawParam::default());
                }
                else if cell.question_marked {
                    let color = consts::QUESTION_MARK_COLOR;
                    let text = Text::new(
                        TextFragment::new("?")
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

    pub fn mouse_motion_event(&mut self, x_pos: f32, y_pos: f32) {
        if let Some((cell_x, cell_y)) = Self::cell_position(x_pos, y_pos) {
            self.grid.set_hovered(cell_x, cell_y, true);
            if let Some((last_cell_x, last_cell_y)) = self.last_hovered_cell {
                if last_cell_x != cell_x || last_cell_y != cell_y {
                    self.grid.set_hovered(last_cell_x, last_cell_y, false);
                    self.grid.set_clicked(last_cell_x, last_cell_y, false);
                    self.last_hovered_cell = Some((cell_x, cell_y));
                }
            } else {
                self.last_hovered_cell = Some((cell_x, cell_y));
            }
        }
        else if let Some((last_cell_x, last_cell_y)) = self.last_hovered_cell {
            self.grid.set_hovered(last_cell_x, last_cell_y, false);
            self.grid.set_clicked(last_cell_x, last_cell_y, false);
            self.last_hovered_cell = None;
        }
    }

    pub fn mouse_button_down_event(&mut self, x_pos: f32, y_pos: f32) {
        if let Some((cell_x, cell_y)) = Self::cell_position(x_pos, y_pos) {
            self.grid.set_clicked(cell_x, cell_y, true);
        }
    }

    pub fn mouse_button_up_event(&mut self, x_pos: f32, y_pos: f32) -> GameState {
        if let Some((cell_x, cell_y)) = Self::cell_position(x_pos, y_pos) {
            self.grid.set_clicked(cell_x, cell_y, false);
            let cell = self.grid.get(cell_x, cell_y);

            if cell.cleared {
                return GameState::Playing;
            }
            if let Err(_) = self.grid.set_cleared(cell_x, cell_y) {
                return GameState::Lose;
            }
            if self.grid.all_cleared() {
                return GameState::Win;
            }
        }
        GameState::Playing
    }

    pub fn mouse_enter_or_leave(&mut self, entered: bool) {
        if !entered {
            if let Some((last_cell_x, last_cell_y)) = self.last_hovered_cell {
                self.grid.set_hovered(last_cell_x, last_cell_y, false);
                self.grid.set_clicked(last_cell_x, last_cell_y, false);
                self.last_hovered_cell = None;
            }
        }
    }

    fn move_from_to(&mut self, from: (usize, usize), to: (usize, usize)) {
        self.grid.set_hovered(to.0, to.1, true);
        self.last_hovered_cell = Some(to);
        self.grid.set_hovered(from.0, from.1, false);
        self.grid.set_clicked(from.0, from.1, false);
    }

    pub fn handle(&mut self, action: Action) -> GameState {

        if let Some((x, y)) = self.last_hovered_cell {
            match action {
                Action::Clear => {
                    if let Err(_) = self.grid.set_cleared(x, y) {
                        return GameState::Lose;
                    }
                    if self.grid.all_cleared() {
                        return GameState::Win;
                    }
                }
                Action::Flag => self.grid.toggle_flagged(x, y),
                Action::QuestionMark => self.grid.toggle_question_marked(x, y),
                Action::ClearAdjacent => {
                    if let Err(_) = self.grid.clear_adjacent(x, y) {
                        return GameState::Lose;
                    }
                    if self.grid.all_cleared() {
                        return GameState::Win;
                    }
                },
                _ => {},
            }
        }
        if let Action::Move(direction) = action {
            let (x, y) = self.last_hovered_cell.unwrap_or((0, 0));
            match direction {
                Direction::Left => {
                    if x > 0 {
                        self.move_from_to((x, y), (x-1, y));
                    }
                },
                Direction::Right => {
                    if x < self.grid.get_shape().0 - 1 {
                        self.move_from_to((x, y), (x+1, y));
                    }
                },
                Direction::Up => {
                    if y > 0 {
                        self.move_from_to((x, y), (x, y-1));
                    }
                },
                Direction::Down => {
                    if y < self.grid.get_shape().1 - 1 {
                        self.move_from_to((x, y), (x, y+1))
                    }
                },
            }
        }
        GameState::Playing
    }
}
