use crate::minezweeper::Level;
use chrono::{DateTime, Local};
use csv::WriterBuilder;
use ggez::input::keyboard::KeyCode;
use std::fs::OpenOptions;
use std::io::Error;

use super::game::GameState;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum Action {
    Move(Direction),
    Clear,
    Flag,
    QuestionMark,
    ClearAdjacent,
    None,
}

pub struct Controls {
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub clear: KeyCode,
    pub flag: KeyCode,
    pub question_mark: KeyCode,
    pub clear_adjacent: KeyCode,
}

impl Controls {
    pub fn default() -> Self {
        Controls {
            up: KeyCode::Up,
            down: KeyCode::Down,
            left: KeyCode::Left,
            right: KeyCode::Right,
            clear: KeyCode::Space,
            flag: KeyCode::C,
            question_mark: KeyCode::Z,
            clear_adjacent: KeyCode::X,
        }
    }

    pub fn handle(&self, keycode: KeyCode) -> Action {
        match keycode {
            _ if keycode == self.up => Action::Move(Direction::Up),
            _ if keycode == self.down => Action::Move(Direction::Down),
            _ if keycode == self.left => Action::Move(Direction::Left),
            _ if keycode == self.right => Action::Move(Direction::Right),
            _ if keycode == self.clear => Action::Clear,
            _ if keycode == self.flag => Action::Flag,
            _ if keycode == self.question_mark => Action::QuestionMark,
            _ if keycode == self.clear_adjacent => Action::ClearAdjacent,
            _ => Action::None,
        }
    }
}

#[derive(Debug)]
pub struct Score {
    pub level: Level,
    pub game_state: GameState,
    pub time: f32,
    pub date_time: DateTime<Local>,
}

impl Score {
    pub fn new(level: Level, game_state: GameState, time: f32) -> Self {
        Score {
            level,
            game_state,
            time,
            date_time: Local::now(),
        }
    }

    pub fn all() -> Result<Vec<Self>, Error> {
        let mut scores = Vec::new();
        let mut reader = csv::Reader::from_path("scores.csv")?;
        for result in reader.records() {
            let record = result?;
            let level_field = record
                .get(0)
                .ok_or(Error::new(std::io::ErrorKind::InvalidData, "Invalid level"))?;
            let level = match level_field {
                "Easy" => Some(Level::Easy),
                "Medium" => Some(Level::Medium),
                "Hard" => Some(Level::Hard),
                _ => None,
            }
            .ok_or(Error::new(std::io::ErrorKind::InvalidData, "Invalid level"))?;

            let game_state_field = record.get(1).ok_or(Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid game state",
            ))?;
            let game_state = match game_state_field {
                "Won" => Some(GameState::Won),
                "Lost" => Some(GameState::Lost),
                "Abandoned" => Some(GameState::Abandoned),
                _ => None,
            }
            .ok_or(Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid game state",
            ))?;

            let time: f32 = record
                .get(2)
                .ok_or(Error::new(std::io::ErrorKind::InvalidData, "Invalid time"))?
                .parse()
                .map_err(|_| Error::new(std::io::ErrorKind::InvalidData, "Invalid time"))?;

            let date_time: DateTime<Local> = record
                .get(3)
                .ok_or(Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid date time",
                ))?
                .parse()
                .map_err(|_| Error::new(std::io::ErrorKind::InvalidData, "Invalid date time"))?;

            scores.push(Score {
                level,
                game_state,
                time,
                date_time,
            });
        }
        Ok(scores)
    }

    pub fn write_to_file(&self) -> Result<(), Error> {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("scores.csv")?;
        let mut csv_writer = WriterBuilder::new().from_writer(file);

        csv_writer.write_record(&[
            self.level.level_info().name,
            self.game_state.to_string(),
            self.time.to_string(),
            self.date_time.to_string(),
        ])?;
        todo!("If file is created new must add the header");
        Ok(())
    }
}
