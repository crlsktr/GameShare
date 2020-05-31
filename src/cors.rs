use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::env;

pub fn add_cors(rocket : rocket::Rocket) -> rocket::Rocket{

	let build_mode = env::var("BUILDMODE");
	let build_mode = match build_mode {
		Ok(r) => r,
		Err(e) => {
			eprintln!("BUILDMODE missing defaulting to \"Dev\", error: {:?}", e);
			String::from("dev")
		},
	};
	
	if build_mode == "dev"
	{
		return rocket;
	}
	
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