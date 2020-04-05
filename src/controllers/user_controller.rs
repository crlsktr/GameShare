use rocket::Route;
use serde::Deserialize;
use rocket_contrib::json::{Json};

#[derive(FromForm, Deserialize)]
struct LoginUser{
	_username: String,
	_pass_phrase: String,
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

pub fn get_user_routes() -> Vec<Route>{
	routes![check_username]
}