mod llama;
mod spawner;
mod collider;

use bevy::prelude::*;
use llama::*;

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {

    // Temp Wall Stuff
    let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    let wall_thickness = 10.0;
    let bounds = Vec2::new(900.0, 600.0);

    commands
        // Cameras
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())

        // Spawn Wall
        .spawn(SpriteComponents {
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(bounds.x() / 2.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y() + wall_thickness)),
            ..Default::default()
        })
        .with(collider::Colider::Solid)

        // Spawn Llama
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlases.add(spawner::spawn(&asset_server)),
            ..Default::default()
        })
        .with(Llama {
            moving_direction: LlamaDirection::E,
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