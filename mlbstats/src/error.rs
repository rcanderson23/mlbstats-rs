use thiserror::Error;

#[derive(Error, Debug)]
pub enum MLBStatsError {
    #[error("request error")]
    ReqwestError(#[from] reqwest::Error),

    #[error("failed to convert MLBTeam")]
    MLBTeamConversionError,

    #[error("failed to convert {0}: {1} is not a {0}")]
    MLBConversionError(String,String),

    #[error("failed to convert league: {0} is not a league")]
    MLBLeagueConversionError(String),

    #[error("failed to serialize params")]
    ParamsError(#[from] serde_url_params::error::Error),
}
