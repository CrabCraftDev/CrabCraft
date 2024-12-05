use evenio::component::Component;

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug, Default)]
pub struct LastTeleport {
    pub id: i32,
    pub confirmed: bool,
}
