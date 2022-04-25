use clap::StructOpt;
use mlbstats_cli::{commands::{self, Commands}, games::scoreboard, standings::standings, league_leaders::league_leaders};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = commands::MLBStats::parse();

    match args.command {
        Commands::Scoreboard(scoreboard_opts) => {
            scoreboard(scoreboard_opts).await?
        },
        Commands::Standings(standings_opts) => {
            standings(standings_opts).await?
        },
        Commands::LeagueLeaders(leader_opts) => {
            league_leaders(leader_opts).await?
        },
    }
    Ok(())
}

