#![allow(non_snake_case)]
use serde::Deserialize;

use crate::games::Game;

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct ScheduleResponse {
    /// Number of games in response
    pub total_games: u32,
    /// Range of dates containing vector of games
    pub dates: Vec<Date>,
}

impl ScheduleResponse {
    pub fn games(&self) -> Vec<Game> {
        let mut games = vec![];
        for date in self.dates.clone() {
            for game in date.games {
                games.push(game)
            }
        }
        games
    }
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct Date {
    /// Date in YYYY-MM-DD format
    pub date: String,
    /// Total games on the date
    pub total_games: u32,
    /// List of games
    pub games: Vec<Game>,
}

