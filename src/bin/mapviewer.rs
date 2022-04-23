use std::fs::File;

use bevy::{prelude::*, DefaultPlugins, window::Windows, sprite::{TextureAtlasBuilder, TextureAtlas}, math::IVec3, render::camera::{ActiveCamera, Camera2d}};
use bevy_simple_tilemap::prelude::*;
use glob1rs::legacy::{map, sprites};

struct MapFileName(String);

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    // mut state: ResMut<ViewerState>,
    // windows: ResMut<Windows>,
	map_file_name: Res<MapFileName>
) {
	// Load terrain images into a texture atlas
	let glob1images = sprites::load();
	let mut terrain_atlas_builder = TextureAtlasBuilder::default();
	let terrain_handles = glob1images
		.into_iter()
		.skip(192)
		.take(164)
		.map(|image| {
			let handle = images.add(image);
			let image = images.get(handle.clone()).unwrap();
			terrain_atlas_builder.add_texture(handle.clone(), image);
			handle
		})
		.collect::<Vec<_>>();
    let terrain_atlas = terrain_atlas_builder
		.finish(&mut images)
		.unwrap();

	// Create a new tilemap for terrain
	let map_file_name = &map_file_name.0;
	let file = File::open(map_file_name).expect("Cannot open map filename");
	let map = map::load(file).expect("Error reading map");
	let tiles: Vec<_> = map
		.into_iter()
		.enumerate()
		.flat_map(|(x, col)| {
			let terrain_atlas = &terrain_atlas;
			let terrain_handles = &terrain_handles;
			col.into_iter()
				.enumerate()
				.map(move |(y, terrain)| {
					let sprite_index = terrain_atlas
						.get_texture_index(&terrain_handles[terrain as usize])
						.unwrap() as u32
					;
					(
						IVec3::new(x as i32, -(y as i32), 0),
						Some(Tile { sprite_index, ..Default::default() })
					)
				})
		})
		.collect();
	let mut tilemap = TileMap::default();
	tilemap.set_tiles(tiles);

	// Show terrain
	let terrain_atlas_handle = texture_atlases.add(terrain_atlas);
	let terrain_bundle = TileMapBundle {
		tilemap,
		texture_atlas: terrain_atlas_handle,
        ..Default::default()
    };
	let mut camera = OrthographicCameraBundle::new_2d();
	camera.transform.translation = Vec3::new(512.0 * 32.0, -512.0 * 32.0, 0.0);
	commands.spawn_bundle(camera);
    commands.spawn_bundle(terrain_bundle);
}

// Inspired by: https://github.com/forbjok/bevy_simple_tilemap/blob/master/examples/simple.rs
fn input_system(
    active_camera: Res<ActiveCamera<Camera2d>>,
    mut camera_transform_query: Query<(&mut Transform,), With<Camera2d>>,
    // mut tilemap_visible_query: Query<&mut Visibility, With<TileMap>>,
    keyboard_input: Res<Input<KeyCode>>,
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

            /*if keyboard_input.just_pressed(KeyCode::V) {
                // Toggle visibility
                let mut visible = tilemap_visible_query.iter_mut().next().unwrap();
                visible.is_visible = !visible.is_visible;
            }*/
        }
    }
}

fn main() {
	let file_name = std::env::args().nth(1).expect("Missing map filename");

	App::new()
		// Disable MSAA, as it produces weird rendering artifacts
        .insert_resource(Msaa { samples: 1 })
		.add_plugins(DefaultPlugins)
		.add_plugin(SimpleTileMapPlugin)
        .insert_resource(MapFileName(file_name))
		.add_system(input_system)
        .add_startup_system(setup)
        .run();
}