use crab_macros::Config;
use serde::Serialize;

#[derive(Config, Serialize, Debug)]
pub struct MainConfig {
    /// The IP address & port used for connecting to the server.
    #[default_value("0.0.0.0".to_string())]
    pub ip: String,
    #[default_value(25565)]
    pub port: i32,

    /// Server brand that is visible for users on F3 screen
    #[default_value("CrabCraft".to_string())]
    pub server_brand: String,

    /// Determines whether to send to players debug information, such as link to
    /// CrabCraft bug tracker or current version of CrabCraft
    ///
    /// *But please, leave it to true and be proud CrabCraft user :3*
    pub mention_crab_craft_usage: bool,

    /// The current configuration version.
    /// **Do not modify this value, as it may cause configuration corruption.**
    #[default_value("0.0.4".to_string())]
    pub _version: String,
}

impl MainConfig {
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}
