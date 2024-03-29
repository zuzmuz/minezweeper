use std::collections::HashMap;

use ggez::graphics::{Canvas, TextAlign, TextLayout};
use ggez::{Context, GameResult};

use crate::consts;
use crate::minezweeper::{draw_text, game::GameState, menu::LEVELS, settings::Score, Level};

#[derive(Debug)]
struct Statistic {
    pub played: usize,
    pub won: usize,
    pub lost: usize,
    pub abandoned: usize,
    pub best_time: Option<f32>,
    pub average_time: Option<f32>,
}

pub struct Scores {
    total_stats: Statistic,
    stats: HashMap<Level, Statistic>,
    error: Option<String>,
}

impl Scores {
    pub fn standard() -> Self {
        match Score::all() {
            Ok(scores) => {
                let mut stats: HashMap<Level, Statistic> = HashMap::new();
                for level in LEVELS.iter() {
                    let scores = scores.iter().filter(|score| score.level == *level);
                    let won_scores = scores
                        .clone()
                        .filter(|score| score.game_state == GameState::Won);
                    let won_scores_count = won_scores.clone().count();
                    stats.insert(
                        *level,
                        Statistic {
                            played: scores.clone().count(),
                            won: won_scores_count,
                            lost: scores
                                .clone()
                                .filter(|score| score.game_state == GameState::Lost)
                                .count(),
                            abandoned: scores
                                .filter(|score| score.game_state == GameState::Abandoned)
                                .count(),
                            best_time: won_scores
                                .clone()
                                .min_by(|&a, &b| a.time.total_cmp(&b.time))
                                .map(|score| score.time),
                            average_time: won_scores
                                .fold(None, |acc, score| Some(acc.unwrap_or(0.0) + score.time))
                                .map(|f| f / (won_scores_count as f32)),
                        },
                    );
                }

                Scores {
                    total_stats: Statistic {
                        played: stats.values().fold(0, |acc, stat| acc + stat.played),
                        won: stats.values().fold(0, |acc, stat| acc + stat.won),
                        lost: stats.values().fold(0, |acc, stat| acc + stat.lost),
                        abandoned: stats.values().fold(0, |acc, stat| acc + stat.abandoned),
                        best_time: None,
                        average_time: None,
                    },
                    stats: stats,
                    error: None,
                }
            }
            Err(error) => Scores {
                total_stats: Statistic {
                    played: 0,
                    won: 0,
                    lost: 0,
                    abandoned: 0,
                    best_time: None,
                    average_time: None,
                },
                stats: HashMap::new(),
                error: Some(error.to_string()),
            },
        }
    }

    fn draw_headers(
        &self,
        canvas: &mut Canvas,
        played: &str,
        won: &str,
        lost: &str,
        abandoned: &str,
        win_percentage: &str,
        best_time: &str,
        average_time: &str,
        y_pos: f32,
    ) -> GameResult {
        draw_text(
            canvas,
            played,
            (consts::SCORES_SCREEN_SIZE.0 * 0.22, y_pos),
            0.7 * consts::QUAD_SIZE.1,
            TextLayout::center(),
            consts::BUTTON_TEXT_COLOR,
        )?;
        draw_text(
            canvas,
            won,
            (consts::SCORES_SCREEN_SIZE.0 * 0.335, y_pos),
            0.7 * consts::QUAD_SIZE.1,
            TextLayout::center(),
            consts::BUTTON_TEXT_COLOR,
        )?;
        draw_text(
            canvas,
            lost,
            (consts::SCORES_SCREEN_SIZE.0 * 0.45, y_pos),
            0.7 * consts::QUAD_SIZE.1,
            TextLayout::center(),
            consts::BUTTON_TEXT_COLOR,
        )?;
        draw_text(
            canvas,
            abandoned,
            (consts::SCORES_SCREEN_SIZE.0 * 0.565, y_pos),
            0.7 * consts::QUAD_SIZE.1,
            TextLayout::center(),
            consts::BUTTON_TEXT_COLOR,
        )?;
        draw_text(
            canvas,
            win_percentage,
            (consts::SCORES_SCREEN_SIZE.0 * 0.68, y_pos),
            0.7 * consts::QUAD_SIZE.1,
            TextLayout::center(),
            consts::BUTTON_TEXT_COLOR,
        )?;

        draw_text(
            canvas,
            best_time,
            (consts::SCORES_SCREEN_SIZE.0 * 0.795, y_pos),
            0.7 * consts::QUAD_SIZE.1,
            TextLayout::center(),
            consts::BUTTON_TEXT_COLOR,
        )?;

        draw_text(
            canvas,
            average_time,
            (consts::SCORES_SCREEN_SIZE.0 * 0.90, y_pos),
            0.7 * consts::QUAD_SIZE.1,
            TextLayout::center(),
            consts::BUTTON_TEXT_COLOR,
        )?;

        Ok(())
    }

    pub fn draw(&self, _ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        if let Some(error) = &self.error {
            draw_text(
                canvas,
                error,
                (
                    0.5 * consts::SCORES_SCREEN_SIZE.0,
                    0.5 * consts::SCORES_SCREEN_SIZE.1,
                ),
                consts::BUTTON_SIZE.1,
                TextLayout::center(),
                consts::BUTTON_TEXT_COLOR,
            )?;
        } else {
            self.draw_headers(
                canvas,
                "P",
                "W",
                "L",
                "A",
                "W%",
                "BT",
                "AT",
                consts::SCORES_SCREEN_SIZE.1 * 0.1,
            )?;
            draw_text(
                canvas,
                "Total",
                (
                    consts::SCORES_SCREEN_SIZE.0 * 0.05,
                    consts::SCORES_SCREEN_SIZE.1 * 0.25,
                ),
                0.7 * consts::QUAD_SIZE.1,
                TextLayout {
                    h_align: TextAlign::Begin,
                    v_align: TextAlign::Middle,
                },
                consts::BUTTON_TEXT_COLOR,
            )?;

            self.draw_headers(
                canvas,
                self.total_stats.played.to_string().as_str(),
                self.total_stats.won.to_string().as_str(),
                self.total_stats.lost.to_string().as_str(),
                self.total_stats.abandoned.to_string().as_str(),
                format!(
                    "{:.1}%",
                    100.0 * (self.total_stats.won as f32) / (self.total_stats.played as f32)
                )
                .as_str(),
                "",
                "",
                consts::SCORES_SCREEN_SIZE.1 * 0.25,
            )?;

            for (i, level) in LEVELS.iter().enumerate() {
                draw_text(
                    canvas,
                    &level.level_info().name,
                    (
                        consts::SCORES_SCREEN_SIZE.0 * 0.05,
                        consts::SCORES_SCREEN_SIZE.1 * (0.45 + (i as f32) * 0.2),
                    ),
                    0.7 * consts::QUAD_SIZE.1,
                    TextLayout {
                        h_align: TextAlign::Begin,
                        v_align: TextAlign::Middle,
                    },
                    consts::BUTTON_TEXT_COLOR,
                )?;
                let stat = &self.stats[level];
                self.draw_headers(
                    canvas,
                    stat.played.to_string().as_str(),
                    stat.won.to_string().as_str(),
                    stat.lost.to_string().as_str(),
                    stat.abandoned.to_string().as_str(),
                    format!("{:.1}%", 100.0 * (stat.won as f32) / (stat.played as f32)).as_str(),
                    stat.best_time
                        .map(|f| format!("{:.1}", f))
                        .unwrap_or("".to_string())
                        .as_str(),
                    stat.average_time
                        .map(|f| format!("{:.1}", f))
                        .unwrap_or("".to_string())
                        .as_str(),
                    consts::SCORES_SCREEN_SIZE.1 * (0.45 + (i as f32) * 0.2),
                )?;
            }
        }
        Ok(())
    }
}
