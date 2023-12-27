use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub db_dsn: String,
    pub db_user: String,
    pub db_pass: String,
}

lazy_static! {
    pub static ref CONFIG: Config = envy::prefixed("CONFIG_")
        .from_env::<Config>()
        .expect("Failed to read config.");
}
