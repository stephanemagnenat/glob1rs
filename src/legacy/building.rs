// see: https://github.com/bevyengine/bevy/discussions/5166
#![allow(clippy::forget_non_drop)]

use bevy::{
    prelude::{Bundle, Commands, Component, Entity, Handle, Image, Transform, UVec2},
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
};
use num_enum::IntoPrimitive;

use super::{
    grid::{grid_to_world, Coord, Grid2D, Rect},
    over_map::{OverMap, OverMapTile},
};

#[derive(Default)]
pub struct BuildingSprites {
    pub texture_atlas: Handle<TextureAtlas>,
    pub sprites: Vec<((Handle<Image>, UVec2), usize)>,
}

#[derive(Component)]
pub struct BuildingPosition {
    pub position: Coord,
    pub size: Coord,
}

#[derive(Bundle)]
pub struct BuildingBundle {
    pub position: BuildingPosition,
    #[bundle]
    pub sprite: SpriteSheetBundle,
}

#[derive(Clone, Copy, Debug, Default, IntoPrimitive)]
#[repr(u8)]
pub enum BuildingLevel {
    #[default]
    Level0,
    Level1,
    Level2,
    Level3,
    Level4,
}

#[derive(Clone, Copy, Debug, Default, IntoPrimitive)]
#[repr(u8)]
pub enum WonderLevel {
    #[default]
    Level0,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
}

#[derive(Clone, Copy, Debug, Default, IntoPrimitive)]
#[repr(u8)]
pub enum ConstructionSiteType {
    #[default]
    Hive,
    Size2,
    Size3,
    Size4,
    Size5,
    Size6,
    Size8,
}

#[derive(Clone, Copy, Debug, Default)]
pub enum BuildingType {
    #[default]
    Hive,
    Hospital(BuildingLevel),
    Inn(BuildingLevel),
    Tower(BuildingLevel),
    School(BuildingLevel),
    Racetrack(BuildingLevel),
    Dojo(BuildingLevel),
    Pool(BuildingLevel),
    Obelisk(BuildingLevel),
    Wonder(WonderLevel),
    ConstructionSite(ConstructionSiteType),
}
impl BuildingType {
    /// Ranges of related images in all images
    pub fn image_ranges() -> Vec<(usize, usize)> {
        vec![
            (370, 49), // normal buildings
            (498, 1),  // hive construction site
            (492, 6),  // various size construction sites
        ]
    }
    /// The index of the corresponding image in BuildingSprites
    pub fn image_index(&self) -> usize {
        match *self {
            BuildingType::Hive => 0,
            BuildingType::Hospital(level) => 1 + 8 * u8::from(level) as usize,
            BuildingType::Inn(level) => 2 + 8 * u8::from(level) as usize,
            BuildingType::Tower(level) => 3 + 8 * u8::from(level) as usize,
            BuildingType::School(level) => 4 + 8 * u8::from(level) as usize,
            BuildingType::Racetrack(level) => 5 + 8 * u8::from(level) as usize,
            BuildingType::Dojo(level) => 6 + 8 * u8::from(level) as usize,
            BuildingType::Pool(level) => 7 + 8 * u8::from(level) as usize,
            BuildingType::Obelisk(level) => 8 + 8 * u8::from(level) as usize,
            BuildingType::Wonder(level) => 40 + u8::from(level) as usize,
            BuildingType::ConstructionSite(ty) => 49 + u8::from(ty) as usize,
        }
    }
    // Currently all buildings are square, and controlled by their x-axis length
    pub fn tile_size(&self, building_sprites: &BuildingSprites) -> i16 {
        let index = self.image_index();
        let size = (building_sprites.sprites[index].0).1.to_array()[0];
        assert_eq!(size & 0x01f, 0);
        (size / 32) as i16
    }
}

impl BuildingBundle {
    pub fn try_spawn(
        position: Coord,
        ty: BuildingType,
        building_sprites: &BuildingSprites,
        over_map: &mut OverMap,
        commands: &mut Commands,
    ) -> Option<Entity> {
        let sprite_index = ty.image_index();
        let side_len = ty.tile_size(building_sprites);
        let size = Coord::new(side_len, side_len);
        let rect = Rect::new(position, size);
        if !over_map.rect_has_value(rect, OverMapTile::Empty) {
            return None;
        }
        let id = commands
            .spawn()
            .insert_bundle(BuildingBundle {
                position: BuildingPosition { position, size },
                sprite: SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(sprite_index),
                    texture_atlas: building_sprites.texture_atlas.clone(),
                    transform: Transform::from_translation(grid_to_world(position)),
                    ..Default::default()
                },
            })
            .id();
        over_map.set_rect_value(rect, OverMapTile::Building(id));
        // TODO: later use team as an entity
        Some(id)
    }
}
