use std::sync::{Arc, Mutex};
use std::thread;

mod llama;
mod spawn;
mod collider;

use bevy::prelude::*;
use llama::*;
use spawn::*;

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut spawn_llama_events: ResMut<Events<SpawnLlama>>,
) {

    // let llama_left_handle: Handle<TextureAtlas> = asset_server.load("models/Llama_left.png");
    // let llama_right_handle: Handle<TextureAtlas> = asset_server.load("models/Llama_right.png");
    asset_server.load_folder("models").unwrap();
    
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
            transform: Transform::from_translation(Vec3::new(250., 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y() + wall_thickness)),
            ..Default::default()
        })
        .with(collider::Colider::Solid);

        // Send Spawn Event
        spawn_llama_events.send(SpawnLlama {
            moving_direction: LlamaDirection::E
        });    
}

fn main() {
    App::build()
        // Main Settings
        .add_resource(WindowDescriptor {
            resizable: true,
            title: "World of Llamas".to_string(),
            ..Default::default()
        })
        .add_event::<SpawnLlama>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup.system())  
        .add_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_plugin(LlamaPlugin)
        .add_plugin(SpawnEventPlugin)
        .run();
}