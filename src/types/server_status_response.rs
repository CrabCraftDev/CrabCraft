use serde::Serialize;

#[derive(Serialize)]
pub struct Version {
    pub name: String,
    pub protocol: u16,
}

#[derive(Serialize)]
pub struct Players {
    pub max: i64,
    pub online: i64,
    pub sample: Vec<String>,
}

#[derive(Serialize)]
pub struct Description {
    pub text: String,
}

#[derive(Serialize)]
pub struct ServerStatusResponse {
    pub version: Version,
    pub players: Players,
    pub description: Description,
    pub enforces_secure_chat: bool,
    pub previews_chat: bool,
}
