use bevy::{prelude::*, sprite::collide_aabb::{collide, Collision}};
use crate::collider::*;

pub struct LlamaPlugin;

pub enum LlamaDirection {
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
    pub moving_direction: LlamaDirection,
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
    mut llama_query: Query<(&mut Llama, &mut Transform, &mut TextureAtlasSprite)>,
    collider_query: Query<(&Colider, &Transform, &Sprite)>
) {
    for(mut llama, mut llama_transform, mut llama_texture_atlas_sprite) in llama_query.iter_mut() {
        println!("Llama");
        // check collision with walls
        for (collider, collision_transform, collision_sprite) in collider_query.iter() {
            let collision = collide(
                llama_transform.translation,
                Vec2::new(30., 30.),
                collision_transform.translation,
                collision_sprite.size
            );

            if let Some(_) = collision {
                *llama_transform.translation.x_mut() -= 0.5
            }
            else {
                *llama_transform.translation.x_mut() += 0.5;
            }
        }
        
    }
}

impl Plugin for LlamaPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(animate_llama.system())
            .add_system(move_llama.system());
    }
}