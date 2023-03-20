use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub base_url: String,
}

impl Config {
    pub fn new() -> Config {
        let base_url = if env::var("BASE_URL").is_ok() {
            env::var("BASE_URL").unwrap()
        } else {
            "http://127.0.0.1:7403".to_string()
        };

        Config { base_url }
    }
}
