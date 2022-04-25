use crate::{
    teams::Team,
    types::{IdNameLink, Record},
};
use serde::Deserialize;

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct StandingsResponse {
    pub records: Vec<StandingsRecord>,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct StandingsRecord {
    pub standings_type: String,
    pub league: IdNameLink,
    pub division: IdNameLink,
    pub team_records: Vec<TeamRecord>,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct TeamRecord {
    pub team: Team,
    pub streak: Streak,
    pub division_rank: String,
    pub league_rank: String,
    pub wild_card_rank: Option<String>,
    pub sport_rank: String,
    pub games_played: u32,
    pub wild_card_games_back: String,
    pub division_games_back: String,
    pub league_record: Record,
    pub records: Split,
    pub runs_allowed: u32,
    pub runs_scored: u32,
    pub wins: u32,
    pub losses: u32,
    pub run_differential: i32,
    pub winning_percentage: String,
    pub does_not_exist: String,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct Streak {
    pub streak_type: String,
    pub streak_number: u32,
    pub streak_code: String,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct Split {
    pub split_records: Vec<Record>,
    pub division_records: Vec<Record>,
    pub overall_records: Vec<Record>,
    pub league_records: Vec<Record>,
    pub expected_records: Vec<Record>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum StandingType {
    RegularSeason,
    WildCard,
    DivisionLeaders,
    WildCardWithLeaders,
    FirstHalf,
    SecondHalf,
    SpringTraining,
    Postseason,
    ByDivision,
    ByConference,
    ByLeague,
    ByOrganization,
}
