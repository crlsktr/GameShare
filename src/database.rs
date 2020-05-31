#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod database;
pub use create_user;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
	let database_url = env::var("DATABASE_URL")
		.expect("DATABASE_URL must be set");
	
	PgConnection::establish((&database_url))
		.expect(&format!("Error connecting to {}", database_url));
}

pub fn create_user(name :String, password:String)
{
	println!(name.to_string(), password.to_string());
}