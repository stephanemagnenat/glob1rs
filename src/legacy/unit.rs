// see: https://github.com/bevyengine/bevy/discussions/5166
#![allow(clippy::forget_non_drop)]

use bevy::{
    prelude::{Bundle, Component, Entity, Handle, Image, Query, Res, ResMut, Transform},
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
};
use rand::seq::IteratorRandom;
use rand::Rng;

use super::{
    direction::Direction,
    grid::{Coord, Grid2D},
    over_map::{OverMap, OverMapTile},
    terrain::{TerrainMap, TerrainType},
};

#[derive(Default)]
pub struct UnitSprites {
    pub texture_atlas: Handle<TextureAtlas>,
    pub sprites: Vec<(Handle<Image>, usize)>,
}

#[derive(Debug)]
pub enum MoveOrder {
    Idle,
    Walk,
    Swim,
}

#[derive(Component)]
pub struct UnitPosition {
    pub position: Coord,
    pub step: u8,
    pub direction: Direction,
    pub order: MoveOrder,
    pub speed: u8,
}

#[derive(Bundle)]
pub struct UnitBundle {
    pub position: UnitPosition,
    #[bundle]
    pub sprite: SpriteSheetBundle,
}

pub fn valid_directions<'a>(
    unit: &'a UnitPosition,
    terrain: &'a TerrainMap,
    over_map: &'a OverMap,
) -> impl Iterator<Item = Direction> + 'a {
    Direction::all().filter(|dir| {
        let position = unit.position + dir.delta();
        TerrainMap::is_in_bounds(position)
            && terrain.passable(position)
            && over_map.get(position) == OverMapTile::Empty
    })
}

pub fn next_order(
    e: Entity,
    unit: &mut UnitPosition,
    terrain: &TerrainMap,
    over_map: &mut OverMap,
    rng: &mut impl Rng,
) {
    // find next position and change animation given terrain
    let dir = valid_directions(unit, terrain, over_map).choose(rng);
    match dir {
        Some(dir) => {
            unit.direction = dir;
            let next_position = unit.position + dir.delta();
            over_map.set(next_position, OverMapTile::Unit(e));
            unit.order = match terrain.get(next_position) {
                TerrainType::Water => MoveOrder::Swim,
                TerrainType::Grass | TerrainType::Sand => {
                    if terrain.get(unit.position) == TerrainType::Water {
                        MoveOrder::Swim
                    } else {
                        MoveOrder::Walk
                    }
                }
                _ => panic!("The next position is not a passable location"),
            };
        }
        None => {
            unit.order = MoveOrder::Idle;
        }
    };
    unit.speed = match unit.order {
        MoveOrder::Idle => 3,
        MoveOrder::Walk => 10,
        MoveOrder::Swim => 5,
    };
}

pub fn move_units(
    unit_sprites: Res<UnitSprites>,
    terrain: Res<TerrainMap>,
    mut over_map: ResMut<OverMap>,
    mut query: Query<(
        Entity,
        &mut UnitPosition,
        &mut TextureAtlasSprite,
        &mut Transform,
    )>,
) {
    let mut rng = rand::thread_rng();
    for (e, mut unit, mut sprite, mut transform) in query.iter_mut() {
        // do movement
        let movement_ended = unit.step as u32 + unit.speed as u32 > 255;
        if movement_ended {
            match unit.order {
                MoveOrder::Idle => {}
                MoveOrder::Walk | MoveOrder::Swim => {
                    over_map.set(unit.position, OverMapTile::Empty);
                    let delta = unit.direction.delta();
                    unit.position += delta;
                    debug_assert_eq!(over_map.get(unit.position), OverMapTile::Unit(e));
                }
            }
        }
        unit.step = unit.step.wrapping_add(unit.speed);
        // update display
        let dir_index = Into::<u8>::into(unit.direction);
        let (delta_position, index) = match unit.order {
            MoveOrder::Idle => (Coord::new(0, 0), (unit.step >> 2) & !0x7),
            MoveOrder::Walk => (
                (unit.direction.delta() * unit.step as i16) / 8,
                dir_index << 3 | unit.step >> 5,
            ),
            MoveOrder::Swim => (
                (unit.direction.delta() * unit.step as i16) / 8,
                64 + (dir_index << 3 | unit.step >> 5),
            ),
        };
        let position = unit.position * 32 + delta_position;
        transform.translation.x = position.x as f32;
        transform.translation.y = -position.y as f32;
        sprite.index = unit_sprites.sprites[index as usize].1;
        // next movement
        if movement_ended {
            next_order(e, &mut unit, &terrain, &mut over_map, &mut rng);
        }
    }
}
