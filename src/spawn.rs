use bevy::prelude::*;
use crate::llama::*;

pub struct SpawnEventPlugin;

pub enum SpawnType {
    Llama,
    Wall
}

pub struct SpawnLlama {
    pub moving_direction: LlamaDirection
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
        let loaded_asset: Handle<Texture>;

        match &self.moving_direction {
            LlamaDirection::E => {
                println!("Moving to Right");
                loaded_asset = asset_server.get_handle("models/Llama_left.png");
            },
            _ => {
                println!("Moving to Left");
                loaded_asset = asset_server.get_handle("models/Llama_right.png");
            }
        }

        let texture = TextureAtlas::from_grid(loaded_asset, Vec2::new(128., 128.), 4, 1);

        commands.spawn(SpriteSheetComponents {
            texture_atlas: texture_atlases.add(texture),
            transform: Transform::from_translation(Vec3::new(
                0., 
                0., 
                0.
            )),
            ..Default::default()
        })
        .with(Llama {
            moving_direction: self.moving_direction.clone(),
            starting_pos_x: 0.,
            starting_pos_y: 0. 
        })
        .with(Timer::from_seconds(0.2, true));
    }
}

// Listens for incoming Events
fn event_spawn_listener(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_reader: Local<EventReader<SpawnLlama>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    spawn_events: ResMut<Events<SpawnLlama>>
)
{
    let mut event_fired = false;
    let mut direction: LlamaDirection = LlamaDirection::N;

    for spawn_event in event_reader.iter(&spawn_events) {
        event_fired = true;
        direction = spawn_event.moving_direction.clone();
    }

    if event_fired {
        println!("Trying to create a new instance");
        let llama = SpawnLlama {
            moving_direction: direction
        };

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