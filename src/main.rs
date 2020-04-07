#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

// use rocket::response::content;
use rocket_contrib::json::{ Json };
use rocket_contrib::serve::StaticFiles;


mod models;
use models::User;

mod controllers;
use controllers::get_user_routes;

fn main() {
	// let my_user = User{ id: 0, name: "hi".to_string()};
	// println!("{}",format!("user: {:?}", my_user));
	rocket::ignite()
		// .mount("/", routes![index,get_user])
		.mount("/user", get_user_routes())
		.mount("/", StaticFiles::from("static/"))
		.launch();
	
}