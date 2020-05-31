#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate dotenv;

use dotenv::dotenv;
use rocket_contrib::serve::StaticFiles;
mod cors;

mod controllers;
use controllers::{get_user_routes,get_game_routes};

fn main() {
	dotenv().ok();
	let mut main_rocket = rocket::ignite()
		.mount("/user", get_user_routes())
		.mount("/game", get_game_routes())
		.mount("/", StaticFiles::from("static/"));

	main_rocket = cors::add_cors(main_rocket);

	main_rocket.launch();
}