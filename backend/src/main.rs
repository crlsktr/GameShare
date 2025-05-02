#[macro_use] extern crate rocket;

#[get("/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/")]
fn ping() -> String {
    format!("hello")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/hello", routes![hello]).mount("/ping", routes![ping])
    
}
