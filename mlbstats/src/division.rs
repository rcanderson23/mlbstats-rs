use std::str::FromStr;
use serde::Deserialize;
use crate::error::MLBStatsError;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DivisionsResponse {
    pub divisions: Vec<Division>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Division {
    /// Unique id for the division
    pub id: u32,

    /// Name of division (e.g. American League West)
    pub name: String,

    /// Season year
    pub season: String,

    /// Shortened name (e.g. AL West)
    pub name_short: String,

    /// Abbreviation of divison (e.g. ALW)
    pub abbreviation: String,
}

#[derive(PartialEq, Debug, Clone)]
pub enum MLBDivision {
    ALCentral, 
    ALEast,
    ALWest,
    NLCentral,
    NLEast,
    NLWest,
}

impl MLBDivision {
    /// Returns the identifier of the league
    pub fn id(&self) -> u32 {
        match self {
            MLBDivision::ALCentral => 202,
            MLBDivision::ALEast => 201,
            MLBDivision::ALWest => 200,
            MLBDivision::NLCentral => 205,
            MLBDivision::NLEast => 204,
            MLBDivision::NLWest => 203,
        }
    }

    /// Returns the full name of the division (e.g. American League West)
    pub fn name(&self) -> String {
        match self {
            MLBDivision::ALCentral => "American League Central".into(),
            MLBDivision::ALEast => "American League East".into(),
            MLBDivision::ALWest => "American League West".into(),
            MLBDivision::NLCentral => "National League Central".into(),
            MLBDivision::NLEast => "National League East".into(),
            MLBDivision::NLWest => "National League West".into(),
        }
    }

    /// Returns the short name of the division (e.g. AL West)
    pub fn short_name(&self) -> String {
        match self {
            MLBDivision::ALCentral => "AL Central".into(),
            MLBDivision::ALEast => "AL East".into(),
            MLBDivision::ALWest => "AL West".into(),
            MLBDivision::NLCentral => "NL Central".into(),
            MLBDivision::NLEast => "NL East".into(),
            MLBDivision::NLWest => "NL West".into(),
        }
    }
}

impl FromStr for MLBDivision {
    type Err = MLBStatsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "alc" => MLBDivision::ALCentral,
            "ale" => MLBDivision::ALEast,
            "alw" => MLBDivision::ALWest,
            "nlc" => MLBDivision::NLCentral,
            "nle" => MLBDivision::NLEast,
            "nlw" => MLBDivision::NLWest,
            "ALC" => MLBDivision::ALCentral,
            "ALE" => MLBDivision::ALEast,
            "ALW" => MLBDivision::ALWest,
            "NLC" => MLBDivision::NLCentral,
            "NLE" => MLBDivision::NLEast,
            "NLW" => MLBDivision::NLWest,
            _ => {
                return Err(MLBStatsError::MLBConversionError(
                    "division".into(),
                    s.into(),
                ))
            }
        })
    }
}

impl From<MLBDivision> for u32 {
    fn from(value: MLBDivision) -> Self {
        match value {
            MLBDivision::ALCentral => 202,
            MLBDivision::ALEast => 201,
            MLBDivision::ALWest => 200,
            MLBDivision::NLCentral => 205,
            MLBDivision::NLEast => 204,
            MLBDivision::NLWest => 203,
        }
    }
}

impl TryFrom<u32> for MLBDivision {
    type Error = MLBStatsError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            200 => MLBDivision::ALWest,
            201 => MLBDivision::ALEast,
            202 => MLBDivision::ALCentral,
            203 => MLBDivision::NLWest,
            204 => MLBDivision::NLEast,
            205 => MLBDivision::NLCentral,
            _ => return Err(MLBStatsError::MLBConversionError("league".into(), value.to_string()))
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn division_conversions() {
        assert_eq!(MLBDivision::from_str("ALC").unwrap(), MLBDivision::ALCentral);
        assert_eq!(MLBDivision::from_str("alc").unwrap(), MLBDivision::ALCentral);

        let alc: u32 = MLBDivision::ALCentral.into();
        assert_eq!(alc, 202);
    }
}
