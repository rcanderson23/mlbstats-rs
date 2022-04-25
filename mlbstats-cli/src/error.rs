use thiserror::Error;

#[derive(Error, Debug)]
pub enum CLIError {
    #[error("command conversion failed")]
    CommandConversionFailed,

    #[error("invalid arg combination: {0}")]
    InvalidArgCombination(String),

    #[error("failed to convert arg: {0}")]
    ArgConversionFailed(String),

    #[error("mlbstats error occurred")]
    MLBStatsError(#[from] mlbstats::error::MLBStatsError),

    #[error("failed to parse date, use YYYY-MM-DD format")]
    DateParseError(#[from] chrono::format::ParseError),
    
    #[error("failed to get response or convert json")]
    JsonConversionError(#[from] reqwest::Error),
}
