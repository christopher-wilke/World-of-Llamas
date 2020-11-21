use bevy::{prelude::*, sprite::collide_aabb::{collide, Collision}};
use crate::collider::*;
use crate::spawn::*;

pub struct LlamaPlugin;

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Llama {
    pub moving_direction: LlamaDirection,
    pub starting_pos_x: f32,
    pub starting_pos_y: f32
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
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut spawn_events: ResMut<Events<SpawnLlama>>,
    mut llama_query: Query<(Entity, &mut Llama, &mut Transform, &Handle<TextureAtlas>)>,
    collider_query: Query<(&Colider, &Transform, &Sprite)>
) {

    for(llama_entity, mut llama, mut llama_transform, mut llama_texture_atlas_handle) in llama_query.iter_mut() {
        // check collision with walls
        for (collider, collision_transform, collision_sprite) in collider_query.iter() {

            let collision = collide(
                llama_transform.translation,
                Vec2::new(30., 30.),
                collision_transform.translation,
                collision_sprite.size
            );
        
            if let Some(_) = collision {

                commands.despawn(llama_entity);

                // Send Spawn Event
                spawn_events.send(SpawnLlama {
                    moving_direction: LlamaDirection::W,
                    starting_pos_x: llama_transform.translation.x() - 0.1, // to do find better way
                    starting_pos_y: llama_transform.translation.y()
                });
            }
            else {
                match llama.moving_direction {
                    LlamaDirection::E => *llama_transform.translation.x_mut() += 0.5,
                    LlamaDirection::W => {
                        *llama_transform.translation.x_mut() -= 0.5;
                    },
                    _ => println!("to implement")
                }
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