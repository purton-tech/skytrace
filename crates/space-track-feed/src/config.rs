use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub space_track_identity: String,
    pub space_track_password: String,
    pub base_url: String,
}

impl Config {
    pub fn new() -> Config {
        let space_track_identity =
            env::var("SPACE_TRACK_IDENTITY").expect("SPACE_TRACK_IDENTITY not set");
        let space_track_password =
            env::var("SPACE_TRACK_PASSWORD").expect("SPACE_TRACK_PASSWORD not set");

        let base_url = if env::var("BASE_URL").is_ok() {
            env::var("BASE_URL").unwrap()
        } else {
            "http://127.0.0.1:7403".to_string()
        };

        Config {
            space_track_identity,
            space_track_password,
            base_url,
        }
    }
}
