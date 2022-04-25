use super::types::LeagueLeaderTypes;
use crate::players::Player;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatsLeadersResponse {
    pub league_leaders: Vec<LeaderCategory>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeaderCategory {
    /// Stat category for the leaders
    pub leader_category: LeagueLeaderTypes,
    /// Season the stats come from
    pub season: Option<String>,
    /// Leaders in the category
    pub leaders: Vec<Leader>,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct Leader {
    /// Rank amoung other leaders
    pub rank: u32,
    /// Represents the value of the leader category (e.g. Number of home runs)
    pub value: String,
    /// League the player plays in
    // TODO: LeagueResponse pub league: ,
    /// Player associated with rank and value
    pub person: Player,
    /// Season the stats come from
    pub season: String,
    /// Number of teams the player played on to accumlate the stat
    pub num_teams: u32,
}
