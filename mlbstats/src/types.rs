#![allow(non_snake_case)]
use serde::Deserialize;

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct Record {
    pub wins: u32,
    pub losses: u32,
    pub ties: Option<u32>,
    pub pct: String,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub division: Option<IdNameLink>,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct IdNameLink {
    pub id: u32,
    pub name: Option<String>,
    pub link: String,
}
