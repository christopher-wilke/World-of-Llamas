use bevy::{prelude::*, sprite::collide_aabb::{collide, Collision}};
use crate::collider::*;
use crate::spawner::*;

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
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
            
                // SPAWN AND DESPAWN
                commands
                    .spawn(SpriteSheetComponents {
                        texture_atlas: texture_atlases.add(spawn(&asset_server, MovingDirection::LlamaMovingLeft)),
                        transform: Transform::from_translation(Vec3::new(
                            llama_transform.translation.x()-1. , 
                            0., 
                            0.
                        )),
                        ..Default::default()
                    })
                    .with(Llama {
                        moving_direction: LlamaDirection::W,
                        name: "Lama 2".to_string()
                    })
                    .with(Timer::from_seconds(0.2, true));

                commands.despawn(llama_entity);
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

fn move_llama_without_detected_collision(direction: LlamaDirection) {

}

impl Plugin for LlamaPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(animate_llama.system())
            .add_system(move_llama.system());
    }
}