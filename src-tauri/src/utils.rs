use crate::models::Player;

pub fn sort_players_initial(players: &mut [Player]) {
    players.sort_by(|a, b| a.rating.cmp(&b.rating).reverse().then(a.name.cmp(&b.name)))
}
