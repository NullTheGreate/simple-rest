use actix_settings::{BasicSettings, NoSettings, Settings};
use serde::Deserialize;

const FILE_PATH: &str = include_str!("../config.toml");

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub db_url: String,
}

pub fn config() -> BasicSettings<NoSettings> {
    Settings::parse_toml("./config.toml").unwrap()
}

pub fn application_config() -> BasicSettings<ApplicationSettings> {
    type CustomSettings = BasicSettings<ApplicationSettings>;
    CustomSettings::from_template(FILE_PATH).unwrap()
}
