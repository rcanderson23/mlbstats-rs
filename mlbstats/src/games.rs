use serde::{Deserialize, Serialize};

use crate::{linescore::Linescore, players::Player, plays::Play, teams::Team, types::Record};

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct Game {
    /// Game ID
    pub game_pk: u32,
    /// Path to Game endpoint
    pub link: String,
    /// Type of game being played (e.g. Regular Season)
    pub game_type: GameTypes,
    /// Teams playing in the game
    pub teams: GameTeams,
    /// Contains various information about the game's status
    pub status: GameStatus,
    /// Information about every inning of the game
    pub linescore: Linescore,
    /// List of all plays that resulted in runs scored
    pub scoring_plays: Vec<Play>,
    /// Most recent play since query
    pub previous_play: Play,
    /// All plays resulting in a home run in the game
    pub home_runs: Vec<Play>,
    /// List of players that played in the game
    pub lineups: Lineup,
}

impl Game {
    pub fn score(&self) -> String {
        format!(
            "{}\n{}: {}\n{}: {}",
            self.game_pk,
            self.teams.away.team.name,
            self.teams.away.score.unwrap_or(0),
            self.teams.home.team.name,
            self.teams.home.score.unwrap_or(0),
        )
    }
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct GameStatus {
    pub abstract_game_state: String,
    pub coded_game_state: String,
    pub detailed_state: String,
    pub status_code: String,
    #[serde(rename = "startTimeTBD")]
    pub start_time_tbd: bool,
    pub abstract_game_code: String,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct GameTeams {
    pub away: GameTeam,
    pub home: GameTeam,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct GameTeam {
    pub score: Option<u8>,
    pub team: Team,
    pub league_record: Record,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GameTypes {
    /// Spring Training
    S,
    /// Regular Season
    R,
    /// Wild Card Game
    F,
    /// Division Series
    D,
    /// League Championship Series
    L,
    /// World Series
    W,
    /// Championship
    C,
    /// Nineteenth Century Series
    N,
    /// Playoffs
    P,
    /// All-Star Game
    A,
    /// Intrasquad
    I,
    /// Exhibition
    E,
}
 
impl Default for GameTypes {
    fn default() -> Self {
        Self::R
    }
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct Lineup {
    home_players: Vec<Player>,
    away_players: Vec<Player>,
}
