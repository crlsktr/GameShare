#[derive(Debug)]
pub struct Config {
	endpoint :i32,
}

pub fn get_config () -> Config {
	Config{endpoint: 8000}
}