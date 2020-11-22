use bevy::prelude::*;
use crate::llama::*;

pub struct SpawnEventPlugin;

pub enum SpawnType {
    Llama,
    Wall
}


pub struct SpawnLlama {
    pub moving_direction: LlamaDirection,
    pub starting_pos_x: f32,
    pub starting_pos_y: f32
}

impl Default for SpawnLlama {
    fn default() -> Self {
        SpawnLlama {
            moving_direction: LlamaDirection::E,
            starting_pos_x: 0.,
            starting_pos_y: 0.
        }
    }
}

trait Spawner {
    fn create_instance(
        &self, 
        mut commands: Commands, 
        asset_server: Res<AssetServer>, 
        mut texture_atlases: ResMut<Assets<TextureAtlas>>
    ) {}
}

impl Spawner for SpawnLlama {
    fn create_instance(
        &self, 
        mut commands: Commands, 
        asset_server: Res<AssetServer>, 
        mut texture_atlases: ResMut<Assets<TextureAtlas>>
    ) {

        let llama1 = SpawnLlama {
            moving_direction: LlamaDirection::E,
            starting_pos_x: 0.,
            starting_pos_y: 0.
        };

        let llama2 = SpawnLlama {
            moving_direction: LlamaDirection::E,
            starting_pos_x: 0.,
            starting_pos_y: 200.
        };

        let mut llamas: Vec<SpawnLlama> = Vec::new();
        llamas.push(llama1);
        llamas.push(llama2);

        while let Some(top) = llamas.pop() {
            let texture_handle = asset_server.load("models/Llama_right.png");
            let texture_atlas = TextureAtlas::from_grid(texture_handle.clone(), Vec2::new(128., 128.), 4, 1);

            commands
                .spawn(SpriteSheetComponents {
                    texture_atlas: texture_atlases.add(texture_atlas),
                    transform: Transform::from_translation(Vec3::new(
                        top.starting_pos_x, 
                        top.starting_pos_y, 
                        0.
                    )),
                    ..Default::default()
                })
                .with(Llama {
                    moving_direction: LlamaDirection::E,
                    starting_pos_x: top.starting_pos_x,
                    starting_pos_y: top.starting_pos_y 
                })
                .with(Timer::from_seconds(0.2, true));
        }
    }
}

// Listens for incoming Events
fn event_spawn_listener(
    commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_reader: Local<EventReader<SpawnLlama>>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    spawn_events: ResMut<Events<SpawnLlama>>
)
{
    let mut event_fired = false;
    let mut llama = SpawnLlama { ..Default::default() };

    for spawn_event in event_reader.iter(&spawn_events) {
        event_fired = true;
        llama.moving_direction = spawn_event.moving_direction.clone();
        llama.starting_pos_x = spawn_event.starting_pos_x;
        llama.starting_pos_y = spawn_event.starting_pos_y;
    }

    if event_fired {
        llama.create_instance(
            commands, 
            asset_server, 
            texture_atlases
        );
    }
}

impl Plugin for SpawnEventPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(event_spawn_listener.system());
    }
}