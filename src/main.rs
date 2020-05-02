#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use dotenv;

// use rocket::response::content;
use rocket_contrib::serve::StaticFiles;
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

mod controllers;
use controllers::{get_user_routes,get_game_routes};

fn main() {
	let build_mode = dotenv::var("BUILDMODE");
	let build_mode = match build_mode {
		Ok(r) => r,
		Err(e) => panic!("missing build_mode env variable {:?}", e),
	};

	// let my_user = User{ id: 0, name: "hi".to_string()};
	// println!("{}",format!("user: {:?}", my_user));
	let mut main_rocket = rocket::ignite()
		// .mount("/", routes![index,get_user])
		.mount("/user", get_user_routes())
		.mount("/game", get_game_routes())
		.mount("/", StaticFiles::from("static/"));

	if build_mode == "Dev"
	{
		main_rocket = add_cors(main_rocket);
	}

	main_rocket.launch();
	
}

fn add_cors(rocket : rocket::Rocket) -> rocket::Rocket{
	let allowed_origins = AllowedOrigins::All;
	let cors = rocket_cors::CorsOptions{
		allowed_origins,
		allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
		allowed_headers: AllowedHeaders::All,
		allow_credentials: true,
		..Default::default()
	}
	.to_cors();

	let cors = match cors {
		Ok(r) => r,
		Err(e) => panic!("Error trying to create a CorsStruct {:?}", e),
	};

	rocket.attach(cors) //attach includes a reference to self (moves the rocket object)
}