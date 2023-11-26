use std::cmp::Ordering;

use crate::models::player::Player;

pub fn sort_players_initial(players: &mut [Player]) {
    players.sort_by(|a, b| {
        a.rating
            .cmp(&b.rating)
            .then(a.title.cmp(&b.title))
            .reverse()
            .then(a.name.cmp(&b.name))
    })
}

pub fn sort_players_ranked(players: &mut [Player]) {
    players.sort_by(|a, b| {
        a.points
            .partial_cmp(&b.points)
            .unwrap_or_else(|| Ordering::Equal)
            .reverse()
            .then(a.name.cmp(&b.name))
    });
}

pub fn helper(players: &mut [(usize, Player)]) {
    players.sort_by(|(_, a), (_, b)| {
        a.rating
            .cmp(&b.rating)
            .then(a.title.cmp(&b.title))
            .reverse()
            .then(a.name.cmp(&b.name))
    })
}
