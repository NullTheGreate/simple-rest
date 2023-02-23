use actix_settings::{BasicSettings, NoSettings, Settings};

pub fn config() -> BasicSettings<NoSettings> {
    Settings::parse_toml("./config.toml").unwrap()
}
