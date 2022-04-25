use std::{str::FromStr, fmt::Display};

use clap::{ArgEnum, Args, Parser, Subcommand};
use mlbstats::{
    league::MLBLeague,
    stats::types::{LeagueLeaderTypes, StatGroup}, teams::MLBTeam,
};

use crate::error::CLIError;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
pub struct MLBStats {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Prints score of one or many games. When no team is provided, all scores are printed.
    Scoreboard(ScoreboardOpts),

    /// Prints standings by division.
    Standings(StandingsOpts),

    /// Prints league leaders.
    LeagueLeaders(LeadersOpts),
}

#[derive(Args)]
pub struct ScoreboardOpts {
    /// Team abbreviation (e.g. TEX or tex)
    #[clap(short, long)]
    pub team: Option<MLBTeam>,

    /// Date in MM-DD-YYYY or YYYY-MM-DD format. Defaults to today's date.
    #[clap(short, long)]
    pub date: Option<String>,

    /// League abbreviation (AL or NL)
    #[clap(short = 'L', long)]
    pub league: Option<MLBLeague>,
}

#[derive(Args)]
pub struct StandingsOpts {
    /// League abbreviation (AL or NL)
    pub league: MLBLeague,

    /// Date in MM-DD-YYYY or YYYY-MM-DD format. Defaults to today's date.
    #[clap(short, long)]
    pub date: Option<String>,
}

#[derive(Args, Clone, Debug)]
pub struct LeadersOpts {
    #[clap(arg_enum)]
    pub stat: Stat,

    /// League abbreviation (AL or NL)
    #[clap(short = 'L', long)]
    pub league: Option<MLBLeague>,

    /// Limit of the results
    #[clap(short = 'l', long, default_value_t = 5)]
    pub limit: u32,
}

#[derive(ArgEnum, Clone, Debug)]
pub enum Stat {
    /// Home Runs
    HR,
    /// On Base Percentage Plus Slugging
    OPS,
    /// On Base Percentage  
    OBP,
    /// Batting Average
    AVG,
    /// Earned Run Average
    ERA,
    /// Saves 
    Saves,
}

impl FromStr for Stat {
    type Err = CLIError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "hr" => Self::HR,
            "ops" => Self::OPS,
            "obp" => Self::OBP,
            "avg" => Self::AVG,
            "era" => Self::ERA,
            "saves" => Self::Saves,
            _ => return Err(CLIError::ArgConversionFailed("stat".into())),
        })
    }
}

impl From<Stat> for LeagueLeaderTypes {
    fn from(s: Stat) -> Self {
        match s {
            Stat::HR => LeagueLeaderTypes::HomeRuns,
            Stat::OPS => LeagueLeaderTypes::OnBasePlusSlugging,
            Stat::OBP => LeagueLeaderTypes::OnBasePercentage,
            Stat::AVG => LeagueLeaderTypes::BattingAverage,
            Stat::ERA => LeagueLeaderTypes::EarnedRunAverage,
            Stat::Saves => LeagueLeaderTypes::Saves,
        }
    }
}

impl Display for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stat::HR => write!(f, "Home Runs"),
            Stat::OPS => write!(f, "On-base Plus Slugging"),
            Stat::OBP => write!(f, "On-base Percentage"),
            Stat::AVG => write!(f, "Batting Average"),
            Stat::ERA => write!(f, "Earned Run Average"),
            Stat::Saves => write!(f, "Saves"),
        }
    }
}

impl Stat {
    pub fn stat_group(&self) -> StatGroup {
        match self {
            Stat::HR => StatGroup::Hitting,
            Stat::OPS => StatGroup::Hitting,
            Stat::OBP => StatGroup::Hitting,
            Stat::AVG => StatGroup::Hitting,
            Stat::ERA => StatGroup::Pitching,
            Stat::Saves => StatGroup::Pitching,
        }
    }
}
