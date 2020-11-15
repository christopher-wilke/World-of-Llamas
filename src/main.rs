mod llama;
mod spawner;

use bevy::prelude::*;
use llama::*;
use spawner::*;

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {

    // to be deleted
    let texture_handle = asset_server.load("Llama_right.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(128., 128.), 4, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        // Cameras
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        
        // Temp Llama
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlases.add(spawner::spawn(&asset_server)),
            ..Default::default()
        })
        .with(Llama {
            moving_direction: Llama_Direction::E,
            name: "Lama 1".to_string()
        })
        .with(Timer::from_seconds(0.2, true));
}

fn main() {
    App::build()
        // Main Settings
        .add_resource(WindowDescriptor {
            resizable: true,
            title: "World of Llamas".to_string(),
            ..Default::default()
        })
        .add_startup_system(startup.system())
        .add_plugins(DefaultPlugins)
        .add_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_plugin(LlamaPlugin)
        .run();
}