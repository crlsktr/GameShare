use serde::Serialize;

#[derive(Debug,Serialize)]
pub struct User{
	pub id : i32,
	pub name: String
}