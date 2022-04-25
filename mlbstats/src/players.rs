use serde::Deserialize;


#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct Player {
    pub id: u32,
    pub full_name: String,
    pub link: String,
    pub first_name: String,
    pub last_name: String,
    pub primary_number: u32,
    pub birth_date: String,
    pub height: String,
    pub weight: String,
    pub active: bool,
    pub captain: bool,
    //TODO: stats, have different types so maybe have to do enum?
    //pub stats: Vec<Stat>,
    /// Date format "YYYY-MM-DD"
    pub mlb_debut_date: String,
    pub bat_side: Side,
    pub pitch_hand: Side,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct Side {
    code: String,
    description: String,
}
