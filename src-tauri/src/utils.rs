use crate::models::{Pairing, Player};

pub fn sort_pairings(pairings: &mut [Pairing]) {
    pairings.sort_by(|a, b| a.number_round.cmp(&b.number_round));
}

pub fn sort_players_initial(players: &mut [Player]) {
    players.sort_by(|a, b| a.rating.cmp(&b.rating).reverse().then(a.name.cmp(&b.name)))
}
