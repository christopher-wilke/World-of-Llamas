use bevy::prelude::*;

pub struct LlamaPlugin;

pub enum Llama_Direction {
    N,
    NE,
    E,
    ES,
    S,
    SW,
    W,
    WN
}

pub struct Llama {
    pub moving_direction: Llama_Direction,
    pub name: String
}

fn animate_llama(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>
) {
    for(timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        if timer.finished {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32; 
        }
    }
}

fn move_llama(
    mut query: Query<(&mut Llama, &mut Transform)>
) {
    for(mut Llama, mut transform) in query.iter_mut() {
        // check collision with walls
        
        *transform.translation.x_mut() += 0.5;
    }
}

impl Plugin for LlamaPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(animate_llama.system())
            .add_system(move_llama.system());
    }
}