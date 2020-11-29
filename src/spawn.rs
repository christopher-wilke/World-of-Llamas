use bevy::prelude::*;
use crate::llama::*;

pub struct SpawnEventPlugin;

pub enum SpawnType {
    Llama,
    Wall
}

pub struct SpawnObject {
    pub llamas: Vec<SpawnLlama>
}

pub struct SpawnLlama {
    pub moving_direction: LlamaDirection,
    pub starting_pos_x: f32,
    pub starting_pos_y: f32
}

impl SpawnLlama {
    fn spawn(
        &self,
        mut commands: Commands, 
        asset_server: &Res<AssetServer>, 
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>
    ) {
        let texture_handle = asset_server.load("models/Llama_right.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle.clone(), Vec2::new(128., 128.), 4, 1);

        commands
            .spawn(SpriteSheetComponents {
                texture_atlas: texture_atlases.add(texture_atlas),
                transform: Transform::from_translation(Vec3::new(
                    self.starting_pos_x, 
                    self.starting_pos_y, 
                    0.
                )),
                ..Default::default()
            })
            .with(Llama {
                moving_direction: LlamaDirection::E,
                starting_pos_x: self.starting_pos_x,
                starting_pos_y: self.starting_pos_y 
            })
            .with(Timer::from_seconds(0.2, true));
    }
}

// Listens for incoming Events
fn llama_spawn_listener(
    commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_reader: Local<EventReader<SpawnObject>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    spawn_events: ResMut<Events<SpawnObject>>
)
{
    for spawn_event in event_reader.iter(&spawn_events) {

        for llama in spawn_event.llamas.iter() {

            let _llama = SpawnLlama {
                moving_direction: llama.moving_direction.clone(),
                starting_pos_x: llama.starting_pos_x,
                starting_pos_y: llama.starting_pos_y
            };
            
            _llama.spawn(
                commands.clone(),
                &asset_server,
                &mut texture_atlases
            );
        }
    }
}

impl Plugin for SpawnEventPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(llama_spawn_listener.system());
    }
}