use crate::consts;
use ggez::graphics::{self, Canvas, DrawParam, PxScale, TextFragment};
use ggez::mint::Point2;
use ggez::{Context, GameResult};

pub struct Button {
    text: String,
    rect: graphics::Rect,
    pub hovered: bool,
    pub clicked: bool,
}

impl Button {
    pub fn new(text: &str, rect: graphics::Rect) -> Button {
        Button {
            text: String::from(text),
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
        let rectangle = graphics::Mesh::new_rounded_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.rect,
            self.rect.h * 0.2,
            color,
        )?;
        canvas.draw(&rectangle, param);
        let text = graphics::Text::new(
            TextFragment::new(self.text.clone()).scale(PxScale::from(consts::BUTTON_SIZE.1)).font("SyneMono"),
        );
        let (x, y) = (self.rect.left() + 0.055555*consts::BUTTON_SIZE.0*((9-self.text.len()) as f32), self.rect.top());
    
        let text_param = graphics::DrawParam::default()
            .dest(Point2 {x: x, y: y})
            .color(consts::BUTTON_TEXT_COLOR);
        canvas.draw(&text, text_param);
        Ok(())
    }
}
