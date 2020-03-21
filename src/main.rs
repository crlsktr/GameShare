#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

// use rocket::response::content;
use rocket_contrib::json::{ Json };
use rocket_contrib::serve::StaticFiles;


mod models;
use models::User;

// mod config;
// use config::get_config;

// #[get("/")]
// fn index() -> String {
// 	format!(r#"Hi {:?}"#, get_config())
// }

#[get("/user")]
fn get_user() -> Option<Json<User>>{
	let user = User{ id: 0, name: "hi".to_string()};
	
	Some(Json(user))
}

#[get("/sec")]
fn get_sec() -> Option<Json<User>>{
	let user = User { id: 1, name: "bye".to_string()};
	Some(Json(user))
}


fn main() {
	// let my_user = User{ id: 0, name: "hi".to_string()};
	// println!("{}",format!("user: {:?}", my_user));
	rocket::ignite()
		// .mount("/", routes![index,get_user])
		.mount("/", routes![get_user])
		.mount("/", StaticFiles::from("static/"))
		.launch();
	
}