use std::fs::File;

use bevy::{DefaultPlugins, window::Windows, sprite::{TextureAtlasBuilder, TextureAtlas, TextureAtlasSprite, SpriteSheetBundle}, math::{IVec3, Vec3}, render::camera::{ActiveCamera, Camera2d}, input::{mouse::{MouseWheel, MouseMotion}, Input}, core::{FixedTimestep, Time}, ecs::schedule::SystemStage, prelude::{Commands, ResMut, Assets, Image, Res, Transform, Query, KeyCode, With, EventReader, MouseButton, App, Msaa, CoreStage, OrthographicCameraBundle}};
use bevy_simple_tilemap::prelude::*;
use glob1rs::legacy::{terrain, sprites, unit::{move_units, UnitPosition, MoveOrder, UnitSprites, UnitBundle}, direction::Direction, grid::Coord, over_map::OverMap};

struct MapFileName(String);

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: ResMut<Windows>,
    map_file_name: Res<MapFileName>
) {
    // Load all images and provide support to create atlases
    let glob1images = sprites::load();
    let mut build_atlas = |skip: usize, take: usize| {
        let mut atlas_builder = TextureAtlasBuilder::default();
        let handles = glob1images
            .iter()
            .skip(skip)
            .take(take)
            .map(|image| {
                let handle = images.add(image.clone());
                let image = images.get(handle.clone()).unwrap();
                atlas_builder.add_texture(handle.clone(), image);
                handle
            })
            // We have to collect because we need to finish the atlas before the next pass
            .collect::<Vec<_>>();
        let atlas = atlas_builder
            .finish(&mut images)
            .unwrap();
        let handles_and_index = handles
            .into_iter()
            .map(|handle| {
                let index = atlas
                    .get_texture_index(&handle)
                    .unwrap();
                (handle, index)
            })
            .collect::<Vec<_>>();
        let atlas_handle = texture_atlases.add(atlas);
        (atlas_handle, handles_and_index)
    };

    // Build unit atlas and handles
    let (unit_atlas_handle, unit_sprites) = build_atlas(0, 192);
    commands.insert_resource(UnitSprites {
        texture_atlas: unit_atlas_handle.clone(),
        sprites: unit_sprites
    });

    // Create a new tilemap for terrain
    let (terrain_atlas_handle, terrain_handles) = build_atlas(192, 164);
    let map_file_name = &map_file_name.0;
    let file = File::open(map_file_name).expect("Cannot open map filename");
    let map = terrain::load(file).expect("Error reading map");
    let tiles: Vec<_> = map.0
        .iter()
        .enumerate()
        .flat_map(|(x, col)| {
            let terrain_handles = &terrain_handles;
            col.iter()
                .enumerate()
                .map(move |(y, &terrain)| {
                    let sprite_index = terrain_handles[terrain as usize].1 as u32;
                    (
                        IVec3::new(x as i32, -(y as i32), 0),
                        Some(Tile { sprite_index, ..Default::default() })
                    )
                })
        })
        .collect();
    let mut tilemap = TileMap::default();
    tilemap.set_tiles(tiles);
    commands.insert_resource(map);

    // Show terrain
    let terrain_bundle = TileMapBundle {
        tilemap,
        texture_atlas: terrain_atlas_handle,
        ..Default::default()
    };
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform.translation = Vec3::new(512.0 * 32.0, -512.0 * 32.0, 10.0);
    commands.spawn_bundle(camera);
    commands.spawn_bundle(terrain_bundle);

    // Create one unit
    commands.spawn().insert_bundle(UnitBundle {
        position: UnitPosition {
            position: Coord::new(0, 0),
            step: 0,
            direction: Direction::Right,
            order: MoveOrder::Idle,
            speed: 3,
        },
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: unit_atlas_handle,
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        }
    });

    // Setup window title
    let window = windows.primary_mut();
    window.set_title(format!("{map_file_name} – Glob1 map viewer"));
}

// Inspired by: https://github.com/forbjok/bevy_simple_tilemap/blob/master/examples/simple.rs
fn input_system(
    active_camera: Res<ActiveCamera<Camera2d>>,
    mut camera_transform_query: Query<(&mut Transform,), With<Camera2d>>,
    // mut tilemap_visible_query: Query<&mut Visibility, With<TileMap>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut motion_evr: EventReader<MouseMotion>,
    buttons: Res<Input<MouseButton>>,
    time: Res<Time>,
) {
    if let Some(active_camera_entity) = active_camera.get() {
        if let Ok((mut tf,)) = camera_transform_query.get_mut(active_camera_entity) {
            let move_speed = 1000.0 * tf.scale.x;
            let zoom_speed = 5.0 * tf.scale.x;

            if keyboard_input.pressed(KeyCode::PageUp) {
                let amount = zoom_speed * time.delta_seconds();
                tf.scale.x = (tf.scale.x - amount).max(0.1);
                tf.scale.y = (tf.scale.y - amount).max(0.1);
            } else if keyboard_input.pressed(KeyCode::PageDown) {
                let amount = zoom_speed * time.delta_seconds();
                tf.scale.x += amount;
                tf.scale.y += amount;
            }

            if keyboard_input.pressed(KeyCode::Left) {
                tf.translation.x -= move_speed * time.delta_seconds();
            } else if keyboard_input.pressed(KeyCode::Right) {
                tf.translation.x += move_speed * time.delta_seconds();
            }

            if keyboard_input.pressed(KeyCode::Down) {
                tf.translation.y -= move_speed * time.delta_seconds();
            } else if keyboard_input.pressed(KeyCode::Up) {
                tf.translation.y += move_speed * time.delta_seconds();
            }

            use bevy::input::mouse::MouseScrollUnit;
            for ev in scroll_evr.iter() {
                let factor = match ev.unit {
                    MouseScrollUnit::Line => ev.y * 12.0,
                    MouseScrollUnit::Pixel => ev.y
                };
                let amount = -zoom_speed * factor * 0.002;
                tf.scale.x = (tf.scale.x + amount).max(0.1);
                tf.scale.y = (tf.scale.y + amount).max(0.1);
            }
            if buttons.pressed(MouseButton::Middle) {
                for ev in motion_evr.iter() {
                    tf.translation.x -= ev.delta.x * tf.scale.x;
                    tf.translation.y += ev.delta.y * tf.scale.y;
                }
            }
        }
    }
}

fn main() {
    let file_name = std::env::args().nth(1).expect("Missing map filename");
    static GLOB1TICK: &str = "glob1tick";

    App::new()
        // Disable MSAA, as it produces weird rendering artifacts
        .insert_resource(Msaa { samples: 1 })
        .add_plugins(DefaultPlugins)
        .add_plugin(SimpleTileMapPlugin)
        .insert_resource(MapFileName(file_name))
        .insert_resource(OverMap::default())
        .add_system(input_system)
        .add_startup_system(setup)
        .add_stage_before(
            CoreStage::Update,
            GLOB1TICK,
            SystemStage::single_threaded()
                .with_run_criteria(FixedTimestep::step(0.03))
        )
        .add_system_to_stage(GLOB1TICK, move_units)
        .run();
}