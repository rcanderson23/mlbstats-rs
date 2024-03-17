use mlbstats::{games::Game, params::QueryParams, schedule::ScheduleResponse};
use tabled::{
    settings::{
        object::{Columns, Rows},
        panel::Header,
        width::MinWidth,
        Alignment, Disable, Modify,
    },
    Table, Tabled,
};

use crate::{commands::ScoreboardOpts, error::CLIError};

#[derive(Tabled)]
struct ScoreLine<'a> {
    name: &'a str,
    runs: &'a u8,
    hits: &'a u8,
    errors: &'a u8,
}

fn create_score_box(game: &Game) -> Table {
    let score = vec![
        ScoreLine {
            name: &game.teams.away.team.name,
            runs: &game.linescore.teams.away.runs,
            hits: &game.linescore.teams.away.hits,
            errors: &game.linescore.teams.away.errors,
        },
        ScoreLine {
            name: &game.teams.home.team.name,
            runs: &game.linescore.teams.home.runs,
            hits: &game.linescore.teams.home.hits,
            errors: &game.linescore.teams.home.errors,
        },
    ];
    Table::new(score)
        .with(Disable::row(Rows::new(..1)))
        .with(MinWidth::new(35))
        .with(Modify::new(Columns::new(1..=2)).with(Alignment::center()))
        .with(Header::new(game_status(game)))
        .to_owned()
}

fn game_status(game: &Game) -> String {
    match game.status.detailed_state.as_str() {
        "In Progress" => format!(
            "{} {}",
            game.linescore.inning_state, game.linescore.current_inning
        ),
        _ => game.status.detailed_state.to_owned(),
    }
}

pub async fn scoreboard(opts: ScoreboardOpts) -> Result<(), CLIError> {
    let games = get_games(opts).await?;

    for game in games {
        println!("{}", create_score_box(&game));
    }
    Ok(())
}

async fn get_games(opts: ScoreboardOpts) -> Result<Vec<Game>, CLIError> {
    let client = mlbstats::client::Client::new();
    let params = QueryParams {
        date: opts.date,
        league_id: opts.league.map(|league| vec![league]),
        team_id: opts.team,
        hydrate: Some("team,linescore".to_string()),
        ..Default::default()
    };
    let games_response = client.get("v1/schedule", Some(&params)).await?;

    let games: ScheduleResponse = games_response.json().await?;
    Ok(games.games())
}
