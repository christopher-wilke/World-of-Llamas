use bevy::prelude::*;

pub enum MovingDirection {
    LlamaMovingRight,
    LlamaMovingLeft
}

struct SpawnStruct {
    spawn_type: MovingDirection
}

trait TextureHandler {
    fn get_handler(&self, asset_server: &Res<AssetServer>) -> TextureAtlas;
}

impl TextureHandler for SpawnStruct {
    fn get_handler(&self, asset_server: &Res<AssetServer>) -> TextureAtlas {
        match self.spawn_type {
            MovingDirection::LlamaMovingRight => {
                let loaded_asset: Handle<Texture> = asset_server.get_handle("models/Llama_right.png");
                TextureAtlas::from_grid(loaded_asset, Vec2::new(128., 128.), 4, 1)
            },
            _ => {
                let texture_handle = asset_server.load("models/Llama_left.png");
                TextureAtlas::from_grid(texture_handle, Vec2::new(128., 128.), 4, 1)
            }
        }
    }
}

pub fn spawn(asset_server: &Res<AssetServer>, direction: MovingDirection) -> TextureAtlas {
    let spawn = SpawnStruct { spawn_type: direction };
    spawn.get_handler(&asset_server)
}