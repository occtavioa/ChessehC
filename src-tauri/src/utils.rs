use crate::models::{Pairing, Player};

pub fn sort_pairings(pairings: &mut [Pairing]) {
    pairings.sort_by(|a, b| match a {
        Pairing::Bye { number_round, .. } => number_round.cmp(match b {
            Pairing::Bye { number_round, .. } => number_round,
            Pairing::Match { number_round, .. } => number_round,
        }),
        Pairing::Match { number_round, .. } => number_round.cmp(match b {
            Pairing::Bye { number_round, .. } => number_round,
            Pairing::Match { number_round, .. } => number_round,
        }),
    });
}

pub fn sort_players_initial(players: &mut [Player]) {
    players.sort_by(|a, b| a.rating.cmp(&b.rating).then(a.name.cmp(&b.name)))
}
