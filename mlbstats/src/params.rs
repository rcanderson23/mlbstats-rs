#![allow(non_snake_case)]
use serde::Serialize;

use crate::{league::MLBLeague, stats::types::{LeagueLeaderTypes, StatGroup}, games::GameTypes, teams::MLBTeam};

/// Stores available params for the MLB API. Not all params are used/available for every endpoint
/// check the endpoint to see available options.
/// Should not need to be directly used unless constructing your own queries.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub league_id: Option<Vec<MLBLeague>>,
    pub date: Option<String>,
    pub season: Option<String>,
    pub standings_types: Option<Vec<String>>,
    pub fields: Option<Vec<String>>,
    pub hydrate: Option<String>,
    pub sport_id: u32,
    pub leader_categories: Option<Vec<LeagueLeaderTypes>>,
    pub leader_game_types: Option<Vec<GameTypes>>,
    pub stat_group: Option<StatGroup>,
    pub limit: Option<u32>,
    pub team_id: Option<MLBTeam>,
}

impl Default for QueryParams {
    fn default() -> Self {
        Self {
            league_id: Default::default(),
            date: Default::default(),
            season: Default::default(),
            standings_types: Default::default(),
            fields: Default::default(),
            hydrate: Default::default(),
            sport_id: 1,
            leader_categories: Default::default(),
            leader_game_types: Default::default(),
            stat_group: Default::default(),
            limit: Default::default(),
            team_id: Default::default(),
        }
    }
}

