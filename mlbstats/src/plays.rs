use serde::Deserialize;

use crate::players::Player;

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct Play {
    pub result: PlayResult,
    pub about: PlayAbout,
    pub count: PlayCount,
    pub matchup: PlayMatchup,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct PlayResult {
    #[serde(rename = "type")]
    pub kind: String,
    pub event: String,
    pub description: String,
    pub rbi: u8,
    pub away_score: u8,
    pub home_score: u8,
}
#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct PlayAbout {
    pub half_inning: String,
    pub inning: u8,
}
#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct PlayCount {
    pub balls: u8,
    pub strikes: u8,
    pub outs: u8,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct PlayMatchup {
    pub batter: Player,
    pub pitcher: Player,
}
