use rocket::Route;
use serde::{Deserialize, Serialize};
use rocket_contrib::json::{Json};

#[derive(FromForm,Serialize, Deserialize, Debug)]
struct Game {
	id: i32,
	name: String,
	instructions: String,
}

#[get("/getGames")]
fn get_games() -> Json<Vec<Game>> {

	Json(vec![])
}

pub fn get_game_routes() -> Vec<Route> {
	return routes![get_games]
}