use ggez::graphics::{self, Canvas};
use ggez::{Context, GameResult};
use super::buttons::Button;
use crate::consts;

pub enum SettingSelected {
    Scores, Controls, None
}

pub struct Settings {
    scores_button: Button,
    controls_button: Button,
}

impl Settings {

    pub fn standard() -> Self {

        let (button_width, button_height) = consts::BUTTON_SIZE;
        let horizontal_margin = 0.5 * (consts::SETTINGS_SCREEN_SIZE.0 - button_width);
        let vertical_margin = 0.33 * (consts::SETTINGS_SCREEN_SIZE.1 - 2.0 * button_height);
        Settings {
            scores_button: Button::new(
                "Scores".to_string(),
                graphics::Rect::new(
                    horizontal_margin, vertical_margin,
                    button_width, button_height,
                )
            ),
            controls_button: Button::new(
                "Controls".to_string(),
                graphics::Rect::new(
                    horizontal_margin, 2.0 * vertical_margin + button_height,
                    button_width, button_height,
                )
            ),
        }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        self.scores_button.draw(ctx, canvas, graphics::DrawParam::default())?;
        self.controls_button.draw(ctx, canvas, graphics::DrawParam::default())?;
        Ok(())
    }

    pub fn mouse_button_down_event(
        &mut self,
        x: f32,
        y: f32,
    ) {
        self.scores_button.clicked = self.scores_button.point_inside(x, y);
        self.controls_button.clicked = self.controls_button.point_inside(x, y);
    }

    pub fn mouse_button_up_event(
        &self,
        x: f32,
        y: f32,
    ) -> SettingSelected {
        if self.scores_button.point_inside(x, y) {
            return SettingSelected::Scores;
        }
        else if self.controls_button.point_inside(x, y) {
            return SettingSelected::Controls;
        }
        SettingSelected::None
    }

    pub fn mouse_motion_event(
        &mut self,
        x: f32,
        y: f32,
    ) {
        self.scores_button.hovered = self.scores_button.point_inside(x, y);
        if !self.scores_button.hovered {
            self.scores_button.clicked = false
        }
        self.controls_button.hovered = self.controls_button.point_inside(x, y);
        if !self.controls_button.hovered {
            self.controls_button.clicked = false
        }
    }
}
