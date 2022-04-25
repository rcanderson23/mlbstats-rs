use chrono::{Datelike, NaiveDate};
use mlbstats::{division::MLBDivision, league::MLBLeague, standings::StandingsResponse};
use tabled::{Header, MinWidth, Table, Tabled};

use crate::{commands::StandingsOpts, error::CLIError};

#[derive(Tabled)]
struct TeamRecord {
    #[tabled(rename = "Team")]
    pub team: String,
    #[tabled(rename = "Record")]
    pub record: String,
    #[tabled(rename = "Games Back")]
    pub games_back: String,
}

fn create_standings_table(division: MLBDivision, teams: Vec<TeamRecord>) -> Table {
    Table::new(teams)
        .with(MinWidth::new(50))
        .with(Header(division.short_name()))
}

pub async fn standings(opts: StandingsOpts) -> Result<(), CLIError> {
    let standings = get_standings(opts.league, opts.date).await?;

    for record in standings.records {
        let division_standings: Vec<TeamRecord> = record
            .team_records
            .iter()
            .map(|team_record| TeamRecord {
                team: team_record.team.name.clone(),
                record: format!("{}-{}", team_record.wins, team_record.losses),
                games_back: team_record.division_games_back.clone(),
            })
            .collect();
        println!(
            "{}",
            create_standings_table(
                MLBDivision::try_from(record.division.id)?,
                division_standings
            )
        );
    }
    Ok(())
}

async fn get_standings(
    league_id: MLBLeague,
    date: Option<String>,
) -> Result<StandingsResponse, CLIError> {
    let client = mlbstats::client::Client::new();

    let date = try_convert_date_string(date)?;
    let season = date.map(|date| date.year().to_string());
    Ok(client.standings(vec![league_id], season, date).await?)
}

// attempts to converts an Option<String> to Option<NaiveDate>
// date format is allowed to be MM-DD-YYYY or YYYY-MM-DD
pub(crate) fn try_convert_date_string(date: Option<String>) -> Result<Option<NaiveDate>, CLIError> {
    if date.is_none() {
        return Ok(None::<NaiveDate>);
    }
    let date = date.unwrap();
    let parsed_date = chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .or_else(|_| chrono::NaiveDate::parse_from_str(&date, "%m-%d-%Y"))?;
    Ok(Some(parsed_date))
}
