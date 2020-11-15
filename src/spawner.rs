use bevy::prelude::*;

enum SpawnType {
    LlamaMovingRight,
    LlamaMovingLeft
}

struct SpawnStruct {
    spawn_type: SpawnType
}

trait TextureHandler {
    fn get_handler(&self, asset_server: &Res<AssetServer>) -> TextureAtlas;
}

impl TextureHandler for SpawnStruct {
    fn get_handler(&self, asset_server: &Res<AssetServer>) -> TextureAtlas {
        match self.spawn_type {
            SpawnType::LlamaMovingRight => {
                let texture_handle = asset_server.load("Llama_right.png");
                TextureAtlas::from_grid(texture_handle, Vec2::new(128., 128.), 4, 1)
            },
            _ => {
                let texture_handle = asset_server.load("Llama_right.png");
                TextureAtlas::from_grid(texture_handle, Vec2::new(128., 128.), 4, 1)
            }
        }
    }
}

pub fn spawn(asset_server: &Res<AssetServer>) -> TextureAtlas {
    let spawn = SpawnStruct {
        spawn_type: SpawnType::LlamaMovingRight
    };
    spawn.get_handler(&asset_server)
}