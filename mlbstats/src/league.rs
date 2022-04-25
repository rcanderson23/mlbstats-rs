use std::str::FromStr;

use serde::Serialize;

use crate::error::MLBStatsError;

#[derive(PartialEq, Debug, Clone)]
pub enum MLBLeague {
    AL,
    NL,
}

impl MLBLeague {
    pub fn id(&self) -> u32 {
        match *self {
            MLBLeague::AL => 103,
            MLBLeague::NL => 104,
        }
    }
}

impl FromStr for MLBLeague {
    type Err = MLBStatsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "al" => MLBLeague::AL,
            "nl" => MLBLeague::NL,
            _ => return Err(MLBStatsError::MLBLeagueConversionError(s.into())),
        })
    }
}

impl TryFrom<u32> for MLBLeague {
    type Error = MLBStatsError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            103 => MLBLeague::AL,
            104 => MLBLeague::NL,
            _ => {
                return Err(MLBStatsError::MLBConversionError(
                    "MLBLeague".into(),
                    value.to_string(),
                ))
            }
        })
    }
}

impl From<MLBLeague> for u32 {
    fn from(value: MLBLeague) -> Self {
        match value {
            MLBLeague::AL => 103,
            MLBLeague::NL => 104,
        }
    }
}

impl Serialize for MLBLeague {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            serializer.serialize_u32(self.id())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn league_conversions() {
        assert_eq!(MLBLeague::try_from(103).unwrap(), MLBLeague::AL);
        assert_eq!(MLBLeague::from_str("al").unwrap(), MLBLeague::AL);
        assert_eq!(MLBLeague::from_str("AL").unwrap(), MLBLeague::AL);
        assert_eq!(MLBLeague::from_str("Al").unwrap(), MLBLeague::AL);

        let al: u32 = MLBLeague::AL.into();
        assert_eq!(al, 103);
    }
}
