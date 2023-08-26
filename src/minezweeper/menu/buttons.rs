use crate::consts;
use ggez::graphics::{Canvas, DrawMode, DrawParam, Mesh, PxScale, Rect, Text, TextFragment};
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
        let text = Text::new(
            TextFragment::new(self.text.clone())
                .scale(PxScale::from(consts::BUTTON_SIZE.1))
                .font("SyneMono"),
        );
        let (x, y) = (
            self.rect.left() + 0.055555 * consts::BUTTON_SIZE.0 * ((9 - self.text.len()) as f32),
            self.rect.top(),
        );

        let text_param = DrawParam::default()
            .dest(Point2 { x: x, y: y })
            .color(consts::BUTTON_TEXT_COLOR);
        canvas.draw(&text, text_param);
        Ok(())
    }

    pub fn point_inside(&self, x: f32, y: f32) -> bool {
        return self.rect.contains(Point2 { x, y });
    }
}
