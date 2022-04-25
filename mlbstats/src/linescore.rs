use serde::Deserialize;

use crate::players::Player;

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct Linescore {
    pub current_inning: u8,
    pub current_inning_ordinal: String,
    pub inning_state: String,
    pub inning_half: String,
    pub is_top_inning: bool,
    pub scheduled_innings: u8,
    pub innings: Vec<Inning>,
    pub defense: Defense,
    pub offense: Offense,
    pub teams: Score,
    pub balls: u8,
    pub strikes: u8,
    pub outs: u8,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct Inning {
    pub num: u8,
    pub ordinal_num: String,
    pub home: RunsHitsErrorsLOB,
    pub away: RunsHitsErrorsLOB,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct Score {
    pub away: RunsHitsErrorsLOB,
    pub home: RunsHitsErrorsLOB,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct RunsHitsErrorsLOB {
    pub runs: u8, 
    pub hits: u8, 
    pub errors: u8, 
    pub left_on_base: u8, 
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct Defense {
    pub pitcher: Player,
    pub catcher: Player,
    pub first: Player,
    pub second: Player,
    pub third: Player,
    pub shortstop: Player,
    pub left: Player,
    pub center: Player,
    pub right: Player,
    pub batter: Player,
    pub on_deck: Player,
    pub in_hole: Player,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct Offense {
    pub batter: Player,
    pub on_deck: Player,
    pub in_hole: Player,
    pub pitcher: Player,
}
