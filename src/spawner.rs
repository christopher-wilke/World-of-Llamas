use bevy::prelude::*;
use crate::llama::*;

pub trait SpawnTrait {
    fn create(&self, mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {}
}

impl SpawnTrait for Llama {
    fn create(&self, mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {

        let loaded_asset: Handle<Texture>;

        match &self.moving_direction {
            LlamaDirection::E => {
                loaded_asset = asset_server.get_handle("models/Llama_right.png");
            },
            _ => {
                loaded_asset = asset_server.get_handle("models/Llama_left.png");
            }
        }

        let texture = TextureAtlas::from_grid(loaded_asset, Vec2::new(128., 128.), 4, 1);

        commands.spawn(SpriteSheetComponents {
            texture_atlas: texture_atlases.add(texture),
            transform: Transform::from_translation(Vec3::new(
                self.starting_pos_x, 
                self.starting_pos_y, 
                0.
            )),
            ..Default::default()
        })
        .with(Llama {
            moving_direction: self.moving_direction.clone(),
            starting_pos_x: self.starting_pos_x,
            starting_pos_y: self.starting_pos_y
        })
        .with(Timer::from_seconds(0.2, true));
    }
}

pub fn create<T>(obj: T, commands: Commands, asset_server: Res<AssetServer>, texture_atlases: ResMut<Assets<TextureAtlas>>) 
    where T: SpawnTrait
{
    obj.create(commands, asset_server, texture_atlases);
}