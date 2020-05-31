use rocket::Route;
use serde::Deserialize;
use rocket_contrib::json::{Json};

#[derive(FromForm, Deserialize, Debug)]
struct LoginUser{
	username: String,
	pass_phrase: String,
}

#[get("/checkUserName?<username>")]
fn check_username(username : Option<String>) ->  Json<bool>{ 
	let name = username.unwrap_or_default();
	if name.to_string() == "me"
	{
		print!("Hi me!");
		return Json(true);
	}
		
	Json(false)
}

#[post("/createUser", format = "json", data="<user>")]
fn create_user(user :Json<LoginUser>) -> Json<bool>{
	println!("{:?}", user);
	Json(true)
}

pub fn get_user_routes() -> Vec<Route>{
	routes![check_username,create_user]
}