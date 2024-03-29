use crate::consts;
use crate::minezweeper::draw_text;
use ggez::graphics::{Canvas, DrawMode, DrawParam, Mesh, Rect, TextLayout};
use ggez::mint::Point2;
use ggez::{Context, GameResult};

pub struct Button {
    text: String,
    rect: Rect,
    pub hovered: bool,
    pub clicked: bool,
}

impl Button {
    pub fn new(text: String, rect: Rect) -> Button {
        Button {
            text,
            rect,
            hovered: false,
            clicked: false,
        }
    }

    pub fn draw(
        &self,
        ctx: &mut Context,
        canvas: &mut Canvas,
        param: impl Into<DrawParam>,
    ) -> GameResult {
        let color = if self.clicked {
            consts::BUTTON_CLICKED_COLOR
        } else if self.hovered {
            consts::BUTTON_HOVERED_COLOR
        } else {
            consts::BUTTON_COLOR
        };
        let rectangle = Mesh::new_rounded_rectangle(
            ctx,
            DrawMode::fill(),
            self.rect,
            self.rect.h * 0.2,
            color,
        )?;
        canvas.draw(&rectangle, param);

        draw_text(
            canvas,
            self.text.clone().as_str(),
            (
                self.rect.left() + 0.5 * self.rect.w,
                self.rect.top() + 0.5 * self.rect.h,
            ),
            self.rect.h,
            TextLayout::center(),
            consts::BUTTON_TEXT_COLOR,
        )?;

        Ok(())
    }

    pub fn point_inside(&self, x: f32, y: f32) -> bool {
        return self.rect.contains(Point2 { x, y });
    }
}
