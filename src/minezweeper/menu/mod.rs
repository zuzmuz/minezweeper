mod buttons;
use crate::consts;
use buttons::Button;
use ggez::{graphics::{self, Canvas}, GameResult, Context};
use ggez::input::{
    keyboard::{KeyCode, KeyInput},
    mouse::MouseButton,
};
use super::minezweeper::{
    MouseMineEventHandler,
    Level
};

pub trait MenuDelegate {
    fn level_selected(&mut self, level: Level);
}

const LEVELS: [Level; 3] = [
    Level::Easy, Level::Medium, Level::Hard
]

pub struct Menu<'a> {
    buttons: [Button; 3],
    delegate: &'a dyn MenuDelegate
}

pub trait ButtonSize {
    fn button_size(&self, button_width: f32, button_height: f32, horizontal_margin: f32, vertical_margin: f32) ->  graphics::Rect;
}

impl ButtonSize for Level {
    fn button_size(&self, button_width: f32, button_height: f32, horizontal_margin: f32, vertical_margin: f32) ->  graphics::Rect {
        match self {
            Self::Easy => {

            },
            Self::Medium => {

            },
            Self::Hard => {
                
            }
        }
    }
}

impl<'a> Menu<'a> {
    pub fn standard(frame_width: f32, frame_height: f32, delegate: &'a impl MenuDelegate) -> Self {
        let (button_width, button_height) = consts::BUTTON_SIZE;
        let horizontal_margin = 0.5 * (frame_width - button_width);
        let vertical_margin = 0.25 * (frame_height - 3.0 * button_height);

        Menu {
            buttons: LEVELS.map(|level|
                Button::new(
                    level.level_info().name,

                )
            ),
            delegate: delegate
        }
    }

    // pub fn set_delegate(&self, delegate: impl MenuDelegate) {
    //     self.delegate = delegate;
    // }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        for button in self.buttons.iter() {
            button.draw(ctx, canvas, graphics::DrawParam::default())?;
        }
        Ok(())
    }
}

impl<'a> MouseMineEventHandler for Menu<'a> {

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        for button in self.buttons.iter_mut() {
            button.clicked = button.point_inside(x, y);
        }
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        for (i, button) in self.buttons.iter().enumerate() {
            if button.point_inside(x, y) {
                self.delegate.level_selected(i)
            }
        }
        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        ctx: &mut Context,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
    ) -> GameResult {
        for button in self.buttons.iter_mut() {
            button.hovered = button.point_inside(x, y);
            if !button.hovered {
                button.clicked = false
            }
        }
        Ok(())
    }
}