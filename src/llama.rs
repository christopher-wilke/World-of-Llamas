use bevy::{prelude::*, sprite::collide_aabb::{collide, Collision}};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
}; 
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

impl Default for Llama {
    fn default() -> Self {
        Llama {
            moving_direction: LlamaDirection::N,
            starting_pos_x: 0.,
            starting_pos_y: 0.
        }
    }
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

impl Distribution<LlamaDirection> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LlamaDirection {
        match rng.gen_range(0, 7) {
            0 => LlamaDirection::N,
            1 => LlamaDirection::NE,
            2 => LlamaDirection::E,
            3 => LlamaDirection::ES,
            4 => LlamaDirection::S,
            5 => LlamaDirection::SW,
            6 => LlamaDirection::W,
            _ => LlamaDirection::WN,
        }
    }
}


fn get_new_llama_direction(dir: LlamaDirection) -> LlamaDirection {

    let random_direction: LlamaDirection = rand::random();    
    random_direction
}

fn new_llama(
    old_llama: Llama,
    old_llama_pos: Transform
) -> Llama {

    let mut new_llama = Llama { ..Default::default()  };

    match old_llama.moving_direction {
        LlamaDirection::N => {
            new_llama.moving_direction = LlamaDirection::W;
        },
        LlamaDirection::NE => {
            new_llama.moving_direction = LlamaDirection::ES;
        },
        LlamaDirection::E => {
            new_llama.moving_direction = LlamaDirection::W;
            new_llama.starting_pos_x = old_llama_pos.translation.x() - 1.;
            new_llama.starting_pos_y = old_llama_pos.translation.y();
        },
        LlamaDirection::ES => {
            new_llama.moving_direction = LlamaDirection::WN;
        },
        LlamaDirection::S => {
            new_llama.moving_direction = LlamaDirection::N;
        },
        LlamaDirection::SW => {
            new_llama.moving_direction = LlamaDirection::NE;
        },
        LlamaDirection::W => {
            new_llama.moving_direction = get_new_llama_direction(LlamaDirection::W);
            new_llama.starting_pos_x = old_llama_pos.translation.x() - 10.;
            new_llama.starting_pos_y = old_llama.starting_pos_y;
        }
        LlamaDirection::WN => {
            new_llama.moving_direction = LlamaDirection::ES;
        }
    }

    new_llama
}

fn move_llama(
    mut commands: Commands,
    mut spawn_llama_events: ResMut<Events<SpawnObject>>,
    mut llama_query: Query<(Entity, &mut Llama, &mut Transform)>,
    collider_query: Query<(&Colider, &Transform, &Sprite)>
) {

    for(llama_entity, llama, mut llama_transform) in llama_query.iter_mut() {
        for (_collider, collision_transform, collision_sprite) in collider_query.iter() {

            let collision = collide(
                llama_transform.translation,
                Vec2::new(30., 30.),
                collision_transform.translation,
                collision_sprite.size
            );
        
            if let Some(_) = collision {

                commands.despawn(llama_entity); // should be improved by just replacing the texture
                let new_llama = new_llama(llama.clone(), *llama_transform);

                // // Trigger Llama Spawn Event
                let mut _llamas: Vec<SpawnLlama> = Vec::new();

                _llamas.push(SpawnLlama {
                    moving_direction: new_llama.moving_direction,
                    starting_pos_x: new_llama.starting_pos_x,
                    starting_pos_y: new_llama.starting_pos_y
                });

                spawn_llama_events.send(SpawnObject {
                    llamas: _llamas
                });
            }
            else {
                match llama.moving_direction {
                    LlamaDirection::N => {
                        *llama_transform.translation.y_mut() += 0.5
                    },
                    LlamaDirection::NE => {
                        *llama_transform.translation.y_mut() += 0.5;
                        *llama_transform.translation.y_mut() += 0.5;
                    },
                    LlamaDirection::E => {
                        *llama_transform.translation.x_mut() += 0.5;
                    },
                    LlamaDirection::ES => {
                        *llama_transform.translation.y_mut() -= 0.5;
                        *llama_transform.translation.x_mut() += 0.5;
                    },
                    LlamaDirection::S => {
                        *llama_transform.translation.y_mut() -= 0.5;
                    },
                    LlamaDirection::SW => {
                        *llama_transform.translation.y_mut() -= 0.5;
                        *llama_transform.translation.x_mut() -= 0.5;
                    },
                    LlamaDirection::W => {
                        *llama_transform.translation.x_mut() -= 0.5;
                    }
                    LlamaDirection::WN => {
                        *llama_transform.translation.y_mut() += 0.5;
                        *llama_transform.translation.x_mut() -= 0.5;
                    }
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