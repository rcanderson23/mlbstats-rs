#![allow(non_snake_case)]
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{error::MLBStatsError, types::IdNameLink};

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct TeamsResponse {
    pub teams: Vec<Team>,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct Team {
    pub all_star_status: String,
    pub id: u32,
    pub name: String,
    pub link: String,
    pub venue: IdNameLink,
    pub team_code: String,
    pub file_code: String,
    pub team_name: String,
    pub location_name: String,
    pub first_year_of_play: String,
    pub league: IdNameLink,
    pub division: IdNameLink,
    pub sport: IdNameLink,
    pub short_name: String,
    pub franchise_name: String,
    pub club_name: String,
    pub active: bool,
}

#[derive(PartialEq, Debug, Clone)]
pub enum MLBTeam {
    ANA,
    ARI,
    ATL,
    BAL,
    BOS,
    CHC,
    CIN,
    CLE,
    COL,
    CWS,
    DET,
    HOU,
    KC,
    LA,
    MIA,
    MIL,
    MIN,
    NYM,
    NYY,
    OAK,
    PHI,
    PIT,
    SD,
    SEA,
    SF,
    STL,
    TB,
    TEX,
    TOR,
    WAS,
}

impl MLBTeam {
    pub fn id(&self) -> u32 {
        match &self {
            MLBTeam::ANA => 108,
            MLBTeam::ARI => 109,
            MLBTeam::ATL => 144,
            MLBTeam::BAL => 110,
            MLBTeam::BOS => 111,
            MLBTeam::CHC => 112,
            MLBTeam::CIN => 113,
            MLBTeam::CLE => 114,
            MLBTeam::COL => 115,
            MLBTeam::CWS => 145,
            MLBTeam::DET => 116,
            MLBTeam::HOU => 117,
            MLBTeam::KC => 118,
            MLBTeam::LA => 119,
            MLBTeam::MIA => 146,
            MLBTeam::MIL => 158,
            MLBTeam::MIN => 142,
            MLBTeam::NYM => 121,
            MLBTeam::NYY => 147,
            MLBTeam::OAK => 133,
            MLBTeam::PHI => 143,
            MLBTeam::PIT => 134,
            MLBTeam::SD => 135,
            MLBTeam::SEA => 136,
            MLBTeam::SF => 137,
            MLBTeam::STL => 138,
            MLBTeam::TB => 139,
            MLBTeam::TEX => 140,
            MLBTeam::TOR => 141,
            MLBTeam::WAS => 120,
        }
    }
}

impl TryFrom<u32> for MLBTeam {
    type Error = MLBStatsError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            108 => MLBTeam::ANA,
            109 => MLBTeam::ARI,
            144 => MLBTeam::ATL,
            110 => MLBTeam::BAL,
            111 => MLBTeam::BOS,
            112 => MLBTeam::CHC,
            113 => MLBTeam::CIN,
            114 => MLBTeam::CLE,
            115 => MLBTeam::COL,
            145 => MLBTeam::CWS,
            116 => MLBTeam::DET,
            117 => MLBTeam::HOU,
            118 => MLBTeam::KC,
            119 => MLBTeam::LA,
            146 => MLBTeam::MIA,
            158 => MLBTeam::MIL,
            142 => MLBTeam::MIN,
            121 => MLBTeam::NYM,
            147 => MLBTeam::NYY,
            133 => MLBTeam::OAK,
            143 => MLBTeam::PHI,
            134 => MLBTeam::PIT,
            135 => MLBTeam::SD,
            136 => MLBTeam::SEA,
            137 => MLBTeam::SF,
            138 => MLBTeam::STL,
            139 => MLBTeam::TB,
            140 => MLBTeam::TEX,
            141 => MLBTeam::TOR,
            120 => MLBTeam::WAS,
            _ => return Err(MLBStatsError::MLBTeamConversionError),
        })
    }
}

impl FromStr for MLBTeam {
    type Err = MLBStatsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "ana" => MLBTeam::ANA,
            "ari" => MLBTeam::ARI,
            "atl" => MLBTeam::ATL,
            "bal" => MLBTeam::BAL,
            "bos" => MLBTeam::BOS,
            "chc" => MLBTeam::CHC,
            "cin" => MLBTeam::CIN,
            "cle" => MLBTeam::CLE,
            "col" => MLBTeam::COL,
            "cws" => MLBTeam::CWS,
            "det" => MLBTeam::DET,
            "hou" => MLBTeam::HOU,
            "kc"  => MLBTeam::KC,
            "la"  => MLBTeam::LA,
            "mia" => MLBTeam::MIA,
            "mil" => MLBTeam::MIL,
            "min" => MLBTeam::MIN,
            "nym" => MLBTeam::NYM,
            "nyy" => MLBTeam::NYY,
            "oak" => MLBTeam::OAK,
            "phi" => MLBTeam::PHI,
            "pit" => MLBTeam::PIT,
            "sd"  => MLBTeam::SD,
            "sea" => MLBTeam::SEA,
            "sf"  => MLBTeam::SF,
            "stl" => MLBTeam::STL,
            "tb"  => MLBTeam::TB,
            "tex" => MLBTeam::TEX,
            "tor" => MLBTeam::TOR,
            "was" => MLBTeam::WAS,
            _ => return Err(MLBStatsError::MLBConversionError("FromStr".into(), s.into())),
        })
    }
}

impl Serialize for MLBTeam {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.id())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_test::{Token, assert_ser_tokens};

    #[test]
    fn mlb_team_conversions() {
        assert_eq!(MLBTeam::try_from(108).unwrap(), MLBTeam::ANA);
        assert_eq!(MLBTeam::try_from(1000).is_err(), true);
    }
    
    #[test]
    fn serialize_mlbteam() {
        let tex = MLBTeam::TEX;
        assert_ser_tokens(&tex, &[Token::U32(140)]);
    }
}
