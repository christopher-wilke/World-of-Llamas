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
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    
    // Temp Wall Stuff
    let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    let wall_thickness = 10.0;
    let bounds = Vec2::new(900.0, 600.0);

    let texture_handle = asset_server.load("models/Llama_right.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle.clone(), Vec2::new(128., 128.), 4, 1);
    let texture_atlas2 = TextureAtlas::from_grid(texture_handle.clone(), Vec2::new(128., 128.), 4, 1);

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

            // .spawn(SpriteSheetComponents {
            //     texture_atlas: texture_atlases.add(texture_atlas),
            //     transform: Transform::from_translation(Vec3::new(
            //         0., 
            //         0., 
            //         0.
            //     )),
            //     ..Default::default()
            // })
            // .with(Llama {
            //     moving_direction: LlamaDirection::E,
            //     starting_pos_x: 0.,
            //     starting_pos_y: 0. 
            // })
            // .with(Timer::from_seconds(0.2, true))

            // .spawn(SpriteSheetComponents {
            //     texture_atlas: texture_atlases.add(texture_atlas2),
            //     transform: Transform::from_translation(Vec3::new(
            //         0., 
            //         250., 
            //         0.
            //     )),
            //     ..Default::default()
            // })
            // .with(Llama {
            //     moving_direction: LlamaDirection::E,
            //     starting_pos_x: 0.,
            //     starting_pos_y: 250. 
            // })
            // .with(Timer::from_seconds(0.2, true));

        // Send Spawn Event
        spawn_llama_events.send(SpawnLlama {
            moving_direction: LlamaDirection::E,
            starting_pos_x: 0.,
            starting_pos_y: 0.
        });

        // // Send Spawn Event
        // spawn_llama_events.send(SpawnLlama {
        //     moving_direction: LlamaDirection::E,
        //     starting_pos_x: 0.,
        //     starting_pos_y: 250.
        // });
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