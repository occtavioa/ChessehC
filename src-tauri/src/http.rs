use serde_json::{Map, Value};
use tauri::api::http::{Client, HttpRequestBuilder, Body};

use crate::models::{player::Player, tournament::Tournament, round::Round, bye::Bye};

pub async fn post_player(player: &Player, client: &Client) -> Result<Value, tauri::api::Error> {
    let mut req_map = Map::new();
    req_map.insert(String::from("tournamentId"), player.tournament_id.into());
    req_map.insert(String::from("name"), player.name.clone().into());
    req_map.insert(String::from("points"), player.points.into());
    req_map.insert(String::from("rating"), player.rating.into());
    req_map.insert(String::from("title"), player.title.clone().into());
    let req = HttpRequestBuilder::new("post", "http://localhost:5000/players")?.body(Body::Json(Value::from(req_map)));
    let res = client.send(req).await?;
    let res_data = res.read().await?;
    Ok(res_data.data)
}

pub async fn post_standing(round: &Round, player: &Player, client: &Client) -> Result<(), tauri::api::Error> {
    let mut req_map = Map::new();
    req_map.insert(String::from("tournamentId"), Value::from(player.tournament_id));
    req_map.insert(String::from("playerId"), Value::from(player.id));
    req_map.insert(String::from("round"), Value::from(round.number));
    req_map.insert(String::from("points"), Value::from(player.points));
    let req = HttpRequestBuilder::new("POST", "http://localhost:5000/standings")?.body(Body::Json(Value::from(req_map)));
    client.send(req).await?;
    Ok(())
}
