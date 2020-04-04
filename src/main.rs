#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

// use rocket::response::content;
use rocket_contrib::json::{ Json };
use serde::Deserialize;
use rocket_contrib::serve::StaticFiles;
use rocket::http::Status;

#[derive(FromForm, Deserialize)]
struct LoginUser{
	username: String,
	pass_phrase: String,
}

mod models;
use models::User;

// mod config;
// use config::get_config;

// #[get("/")]
// fn index() -> String {
// 	format!(r#"Hi {:?}"#, get_config())
// }

#[get("/default")]
fn get_default_user() -> Option<Json<User>>{
	let user = User{ id: 0, name: "hi".to_string()};
	
	Some(Json(user))
}

#[get("/sec")]
fn get_sec() -> Option<Json<User>>{
	let user = User { id: 1, name: "bye".to_string()};
	Some(Json(user))
}

#[post("/checkUserName", format = "json", data = "<user>")]
fn check_username(user : Json<LoginUser>) ->  Json<bool>{
	println!("{}",user.username.to_string());
	if user.username == "me"
	{
		print!("Hi me!");
		return Json(true);
	}
		
	Json(false)
}

fn main() {
	// let my_user = User{ id: 0, name: "hi".to_string()};
	// println!("{}",format!("user: {:?}", my_user));
	rocket::ignite()
		// .mount("/", routes![index,get_user])
		.mount("/user", routes![get_default_user, check_username])
		.mount("/", StaticFiles::from("static/"))
		.launch();
	
}