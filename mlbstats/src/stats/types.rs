use serde::{Deserialize, Serialize};

/// Common parameter for stat queries
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum LeagueLeaderTypes {
    AirOuts,
    Assists,
    AtBats,
    Balk,
    BattingAverage,
    BlownSaves,
    CatcherEarnedRunAverage,
    CatchersInterference,
    CaughtStealing,
    Chances,
    CompleteGames,
    DoublePlays,
    Doubles,
    EarnedRun,
    EarnedRunAverage,
    Errors,
    ExtraBaseHits,
    FieldingPercentage,
    Flyouts,
    GamesFinished,
    GamesPlayed,
    GamesStarted,
    GroundIntoDoublePlays,
    GroundOuts,
    GroundoutToFlyoutRatio,
    HitBatsman,
    HitByPitches,
    Hits,
    HitsPer9Inn,
    Holds,
    HomeRuns,
    Innings,
    InningsPitched,
    IntentionalWalks,
    Losses,
    NumberOfPitches,
    OnBasePercentage,
    OnBasePlusSlugging,
    OutfieldAssists,
    PassedBalls,
    Pickoffs,
    PitchesPerInning,
    PutOuts,
    RangeFactorPer9Inn,
    RangeFactorPerGame,
    Runs,
    RunsBattedIn,
    SacrificeBunts,
    SacrificeFlies,
    SaveOpportunities,
    Saves,
    Shutouts,
    SluggingPercentage,
    StolenBasePercentage,
    StolenBases,
    StrikeoutWalkRatio,
    Strikeouts,
    StrikeoutsPer9Inn,
    ThrowingErrors,
    TotalBases,
    TotalBattersFaced,
    TotalPlateAppearances,
    TriplePlays,
    Triples,
    Walks,
    WalksAndHitsPerInningPitched,
    WalksPer9Inn,
    WildPitch,
    WinPercentage,
    Wins,
}

/// Common parameter for stat queries
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum StatGroup {
    Catching,
    Fielding,
    Game,
    Hitting,
    Pitching,
    Running,
    Streak,
    Team,
}