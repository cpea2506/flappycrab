use bevy::prelude::*;

pub struct StartMessage;

impl StartMessage {
    pub fn texture(asset_server: &Res<AssetServer>) -> Handle<Image> {
        asset_server.load("images/start_message.png")
    }
}
