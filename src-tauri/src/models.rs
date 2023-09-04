use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Tournament {
    pub name: String,
    pub city: String,
    pub fide_federation: String,
    pub date_start: String,
    pub date_end: String,
    pub type_tournament: String,
    pub format: String,
    pub chief_arbiter: String,
    pub deputy_chief_arbiter: String,
    pub time_control: String,
    pub number_rounds: u16,
    pub current_round: Option<u16>,
}

#[derive(Default, Deserialize, Serialize)]
pub struct Player {
    pub name: String,
    pub sex: String,
    pub title: String,
    pub rating: u16,
    pub fide_federation: String,
    pub fide_number: Option<u16>,
    pub birth_date: String,
}
