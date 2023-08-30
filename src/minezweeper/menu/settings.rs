use std::collections::HashMap;

use ggez::graphics::Canvas;
use ggez::{Context, GameResult};

use crate::minezweeper::game::GameState;
use crate::minezweeper::{settings::Score, Level};
use crate::minezweeper::menu::LEVELS;

#[derive(Debug)]
struct Statistic {
    pub played: usize,
    pub won: usize,
    pub lost: usize,
    pub abandoned: usize,
}

pub struct Settings {
    total_stats: Statistic,
    stats: HashMap<Level, Statistic>,
    error: Option<String>,
}

impl Settings {
    pub fn standard() -> Self {
        
        match Score::all() {
            Ok(scores) => {
                println!("{:?}", scores);
                let mut stats: HashMap<Level, Statistic> = LEVELS.into_iter().fold(HashMap::new(), |mut acc, level| {
                    acc.insert(level, Statistic {
                        played: 0,
                        won: 0,
                        lost: 0,
                        abandoned: 0,
                    });
                    acc
                });
        
                for score in scores {
                    stats.get_mut(&score.level).unwrap().played += 1;
                    match score.game_state {
                        GameState::Won => {
                            stats.get_mut(&score.level).unwrap().won += 1;
                        },
                        GameState::Lost => {
                            stats.get_mut(&score.level).unwrap().lost += 1;
                        },
                        GameState::Abandoned => {
                            stats.get_mut(&score.level).unwrap().abandoned += 1;
                        }
                        _ => {},
                    }
                }
                
                for level in LEVELS {
                    println!("{:?}", stats.get(&level));
                }

                Settings {
                    total_stats: Statistic {
                        played: stats.values().fold(0, |acc, stat| acc + stat.played),
                        won: stats.values().fold(0, |acc, stat| acc + stat.won),
                        lost: stats.values().fold(0, |acc, stat| acc + stat.lost),
                        abandoned: stats.values().fold(0, |acc, stat| acc + stat.abandoned)
                    },
                    stats: stats,
                    error: None,
                }
            },
            Err(error) => {
                println!("{:?}", error);
                Settings {
                    total_stats: Statistic {
                        played: 0,
                        won: 0,
                        lost: 0,
                        abandoned: 0,
                    },
                    stats: HashMap::new(),
                    error: Some(error.to_string()),
                }
            }
        }
    }

    pub fn draw(&self, _ctx: &mut Context, _canvas: &mut Canvas) -> GameResult {
        
        let _ = &self.stats;
        let _ = &self.total_stats;
        let _ = &self.error;
        Ok(())
    }
}