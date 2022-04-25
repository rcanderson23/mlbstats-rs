#![allow(non_snake_case)]
use crate::error::MLBStatsError;
use crate::games::GameTypes;
use crate::league::MLBLeague;
use crate::params::QueryParams;
use crate::schedule::ScheduleResponse;
use crate::standings::StandingsResponse;
use crate::stats::stats_leaders::StatsLeadersResponse;
use crate::stats::types::{LeagueLeaderTypes, StatGroup};
use crate::teams::{MLBTeam, TeamsResponse};
use chrono::Datelike;
use reqwest::Response;

const MLB_BASE: &str = "https://statsapi.mlb.com/api/";
#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    /// Creates a new, reusuable client to query the mlbstats endpoints
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Returns the schedule of games for the current day when no date is provided. The results can be 
    /// further narrowed down with provided parameters. 
    ///
    /// Endpoint: https://statsapi.mlb.com/api/v1/schedule
    ///
    /// Hydrations should be provided via a comma separated string (e.g. "linescore,")
    ///
    /// Available hydrations can be found at https://statsapi.mlb.com/api/v1/schedule\?sportId\=1\&hydrate\=hydrations
    pub async fn schedule<D>(
        &self,
        league_id: Option<Vec<MLBLeague>>,
        team_id: Option<MLBTeam>,
        date: Option<D>,
        hydrate: Option<String>,
    ) -> Result<ScheduleResponse, MLBStatsError>
    where
        D: Datelike,
    {
        let params = QueryParams {
            date: option_date_to_string(date),
            league_id,
            team_id,
            hydrate,
            ..Default::default()
        };
        let url = build_url("v1/schedule", Some(&params))?;
        let resp = self
            .client
            .get(url)
            .send()
            .await?
            .json::<ScheduleResponse>()
            .await?;
        Ok(resp)
    }

    /// Returns all MLB Teams
    /// Endpoint: https://statsapi.mlb.com/api/v1/teams
    pub async fn teams(&self) -> Result<TeamsResponse, MLBStatsError> {
        let params = QueryParams {
            league_id: Some(vec![MLBLeague::AL, MLBLeague::NL]),
            ..Default::default()
        };
        let url = build_url("v1/teams", Some(&params))?;
        Ok(self
            .client
            .get(url)
            .send()
            .await?
            .json::<TeamsResponse>()
            .await?)
    }

    pub async fn game_ids(&self) -> Result<Vec<u32>, MLBStatsError> {
        let games = self
            .client
            .get(format!("{}{}", MLB_BASE, "v1/schedule"))
            .query(&[("sportId", "1")])
            .send()
            .await?
            .json::<ScheduleResponse>()
            .await?
            .games();

        let mut game_ids = Vec::new();
        for game in games {
            game_ids.push(game.game_pk)
        }

        Ok(game_ids)
    }

    /// Retrieve the MLB standings based on the provided parameters
    /// Endpoint: https://statsapi.mlb.com/api/v1/standings
    /// Hydrate options: [team, league, divison, sport, conference, record(conference),
    /// record(division), standingsOdds]
    pub async fn standings<D>(
        &self,
        league_id: Vec<MLBLeague>,
        season: Option<String>,
        date: Option<D>,
    ) -> Result<StandingsResponse, MLBStatsError>
    where
        D: Datelike,
    {
        let params = QueryParams {
            date: option_date_to_string(date),
            league_id: Some(league_id),
            season,
            ..Default::default()
        };

        let standings = self.get("v1/standings", Some(&params)).await?;
        Ok(standings.json::<StandingsResponse>().await?)
    }

    /// Endpoint: https://statsapi.mlb.com/api/v1/stats/leaders
    pub async fn stats_leaders(
        &self,
        leader_categories: Vec<LeagueLeaderTypes>,
        leader_game_types: Vec<GameTypes>,
        stat_group: Option<StatGroup>,
        limit: Option<u32>,
        league_id: Option<Vec<MLBLeague>>,
    ) -> Result<StatsLeadersResponse, MLBStatsError> {
        let params = QueryParams {
            leader_categories: Some(leader_categories),
            leader_game_types: Some(leader_game_types),
            stat_group,
            limit,
            league_id,
            ..Default::default()
        };

        let leaders = self.get("v1/stats/leaders", Some(&params)).await?;
        Ok(leaders.json::<StatsLeadersResponse>().await?)
    }

    /// Makes requests to an statsapi endpoint with the provided path and parameters
    ///
    /// hydrate the teams and linescores of the games in the current day's schedule
    /// https://statsapi.mlb.com/api/v1/schedule\?sportId\=1\&hydrate\=team,linescore
    pub async fn get(
        &self,
        path: &str,
        params: Option<&QueryParams>,
    ) -> Result<Response, MLBStatsError> {
        let url = build_url(path, params)?;
        Ok(self.client.get(url).send().await?)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

// Converts and Option<Datelike> -> Option<String> for api usage
fn option_date_to_string<D>(date: Option<D>) -> Option<String>
where
    D: Datelike,
{
    date.map(|date| format!("{:0>2}/{:0>2}/{}", date.month(), date.day(), date.year()))
}

// Unable to use reqwest params option in request builder so we have to build the url ourselves
fn build_url(path: &str, params: Option<&QueryParams>) -> Result<String, MLBStatsError> {
    if params.is_none() {
        return Ok(format!("{}{}", MLB_BASE, path));
    }

    let params = serde_url_params::to_string(params.unwrap())?;
    if params.is_empty() {
        return Ok(format!("{}{}", MLB_BASE, path));
    } else {
        Ok(format!("{}{}?{}", MLB_BASE, path, params))
    }
}

#[cfg(test)]
mod test {
    use super::super::league::MLBLeague;
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn url_building() -> Result<(), MLBStatsError> {
        // no params
        let url = build_url("v1/standings", None)?;
        assert_eq!(url, "https://statsapi.mlb.com/api/v1/standings");

        // empty params
        let params = QueryParams {
            ..Default::default()
        };
        let url = build_url("v1/standings", Some(&params))?;
        assert_eq!(url, "https://statsapi.mlb.com/api/v1/standings?sportId=1");

        // with a param
        let params = QueryParams {
            league_id: Some(vec![MLBLeague::AL]),
            ..Default::default()
        };
        let url = build_url("v1/standings", Some(&params))?;
        assert_eq!(
            url,
            "https://statsapi.mlb.com/api/v1/standings?leagueId=103&sportId=1"
        );

        let params = QueryParams {
            league_id: Some(vec![MLBLeague::AL, MLBLeague::NL]),
            ..Default::default()
        };

        let url = build_url("v1/standings", Some(&params))?;
        assert_eq!(
            url,
            "https://statsapi.mlb.com/api/v1/standings?leagueId=103&leagueId=104&sportId=1"
        );

        Ok(())
    }

    #[tokio::test]
    async fn teams() {
        let client = Client::new();
        let teams = client.teams().await;
        assert!(teams.unwrap().teams.len() == 30);
    }

    #[tokio::test]
    async fn standings() {
        let client = Client::new();
        let standings = client
            .standings(vec![MLBLeague::AL], None, None::<NaiveDate>)
            .await;
        dbg!(&standings);
        assert!(standings.is_ok());

        let standings = client
            .standings(vec![MLBLeague::AL, MLBLeague::NL], None, None::<NaiveDate>)
            .await;
        dbg!(&standings);
        assert!(standings.is_ok());
    }

    #[tokio::test]
    async fn schedule() {
        let client = Client::new();
        let schedule = client.schedule(None, None, None::<NaiveDate>, None).await;
        assert!(schedule.is_ok());

        let schedule = client
            .schedule(None, None, Some(NaiveDate::from_ymd(2022, 04, 12)), None)
            .await;
        assert!(schedule.is_ok());
        assert!(schedule.unwrap().dates.len() != 0);
    }
}
