use super::game::GameState;
use crate::minezweeper::Level;
use chrono::{DateTime, Local};
use csv::WriterBuilder;
use ggez::input::keyboard::KeyCode;
use rusqlite::{Connection, params};
use std::{error::Error, fmt::Display, fs::OpenOptions};

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

#[derive(Clone, Debug, Eq, PartialEq)]
enum ScoreError {
    InvalidLevel,
    InvalidGameState,
    InvalidTime,
    InvalidDateTime,
}

impl Display for ScoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScoreError::InvalidLevel => write!(f, "Invalid level"),
            ScoreError::InvalidGameState => write!(f, "Invalid game state"),
            ScoreError::InvalidTime => write!(f, "Invalid time"),
            ScoreError::InvalidDateTime => write!(f, "Invalid date time"),
        }
    }
}

impl Error for ScoreError {}

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

    fn from(level: String, game_state: String, time: f32, date_time: String) -> Result<Self, ScoreError> {
        Ok(Score {
            level: match level.as_str() {
                "Easy" => Level::Easy,
                "Medium" => Level::Medium,
                "Hard" => Level::Hard,
                _ => return Err(ScoreError::InvalidLevel),
            },
            game_state: match game_state.as_str() {
                "Won" => GameState::Won,
                "Lost" => GameState::Lost,
                "Abandoned" => GameState::Abandoned,
                _ => return Err(ScoreError::InvalidGameState),
            },
            time,
            date_time: date_time.parse().map_err(|_| ScoreError::InvalidDateTime)?,
        })
    }

    #[allow(unused)]
    fn from_csv() -> Result<Vec<Self>, Box<dyn Error>> {
        let mut scores = Vec::new();
        let mut reader = csv::Reader::from_path("scores.csv")?;
        for result in reader.records() {
            let record = result?;
            let level = record.get(0).ok_or(ScoreError::InvalidLevel)?;

            let game_state = record.get(1).ok_or(ScoreError::InvalidGameState)?;
            
            let time: f32 = record
                .get(2)
                .ok_or(ScoreError::InvalidTime)?
                .parse()
                .map_err(|_| ScoreError::InvalidTime)?;

            let date_time = record.get(3).ok_or(ScoreError::InvalidDateTime)?;

            scores.push(Score::from(level.to_string(), game_state.to_string(), time, date_time.to_string())?);
        }
        Ok(scores)
    }

    fn get_sqlite_con() -> Result<Connection, Box<dyn Error>> {
        let new_file = !std::path::Path::new("scores.db").exists();
        let con = Connection::open("scores.db")?;
        
        if new_file {
            con.execute(
                "CREATE TABLE score (
                    id   INTEGER PRIMARY KEY,
                    level TEXT NOT NULL,
                    game_state TEXT NOT NULL,
                    time REAL NOT NULL,
                    date_time TEXT NOT NULL
                )",
                (), // empty list of parameters.
            )?;
        }

        Ok(con)
    }


    fn from_sqlite() -> Result<Vec<Self>, Box<dyn Error>> {
        
        let con = Self::get_sqlite_con()?;

        let mut stmt = con.prepare("SELECT level, game_state, time, date_time FROM score")?;
        let scores_query = stmt.query_map([], |row| {
            Ok(Score::from(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
            ))
        })?;
        
        let mut scores = Vec::new();
        for score in scores_query {
            scores.push(score??);
        }
        Ok(scores)
    }

    pub fn all() -> Result<Vec<Self>, Box<dyn Error>> {
        Self::from_sqlite()
    }

    #[allow(unused)]
    fn save_to_csv(&self) -> Result<(), Box<dyn Error>> {
        let new_file = !std::path::Path::new("scores.csv").exists();

        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("scores.csv")?;

        let mut csv_writer = WriterBuilder::new().from_writer(file);

        if new_file {
            csv_writer.write_record(&["level", "game_state", "time", "date_time"])?;
        }

        csv_writer.write_record(&[
            self.level.level_info().name,
            self.game_state.to_string(),
            self.time.to_string(),
            self.date_time.to_string(),
        ])?;
        Ok(())
    }

    fn save_to_sqlite(&self) -> Result<(), Box<dyn Error>> {
        let con = Self::get_sqlite_con()?;

        con.execute(
            "INSERT INTO score (level, game_state, time, date_time) VALUES (?1, ?2, ?3, ?4)",
            params![&self.level.level_info().name, &self.game_state.to_string(), self.time, &self.date_time.to_string()],
        )?;
        Ok(())
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        self.save_to_sqlite()
    }
}
