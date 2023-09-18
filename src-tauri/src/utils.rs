use crate::models::{Pairing, Player, ByePoint};

pub fn sort_pairings(pairings: &mut [Pairing]) {
    pairings.sort_by(|a, b| a.number_round.cmp(&b.number_round));
}

pub fn sort_players_initial(players: &mut [Player]) {
    players.sort_by(|a, b| a.rating.cmp(&b.rating).reverse().then(a.name.cmp(&b.name)))
}

pub fn sort_players_rating(players: &mut [Player]) {
    players.sort_by(|a, b| a.points.total_cmp(&b.points).reverse().then(a.rating.cmp(&b.rating).reverse()).then(a.name.cmp(&b.name)))
}

pub fn get_bye_point(bye: &ByePoint) -> f64 {
    match bye {
        ByePoint::U => 1.0,
        ByePoint::Z => 0.0,
    }
}
