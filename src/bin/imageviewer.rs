use bevy::prelude::*;
use glob1rs::legacy::sprites;

#[derive(Default)]
struct ViewerState {
    current: usize,
    images: Vec<Handle<Image>>,
}

fn update_title(state: &ViewerState, images: Res<Assets<Image>>, mut windows: ResMut<Windows>) {
    let selected_handle = state.images[state.current].clone();
    let size = images.get(&selected_handle).unwrap().size().as_uvec2();
    let window = windows.primary_mut();
    window.set_title(format!(
        "Image {} / {} - {}",
        state.current,
        state.images.len(),
        size
    ));
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut state: ResMut<ViewerState>,
    windows: ResMut<Windows>,
) {
    let glob1images = sprites::load();
    state.images = glob1images
        .into_iter()
        .map(|image| images.add(image))
        .collect::<Vec<_>>();
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        texture: state.images[state.current].clone(),
        ..default()
    });
    update_title(&state, images.into(), windows);
}

fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    images: Res<Assets<Image>>,
    mut state: ResMut<ViewerState>,
    mut query: Query<&mut Handle<Image>>,
    windows: ResMut<Windows>,
) {
    let mut changed = false;
    if keys.just_pressed(KeyCode::Right) && state.current + 1 < state.images.len() {
        state.current += 1;
        changed = true;
    }
    if keys.just_pressed(KeyCode::PageDown) && state.current + 10 < state.images.len() {
        state.current += 10;
        changed = true;
    }
    if keys.just_released(KeyCode::Left) && state.current >= 1 {
        state.current -= 1;
        changed = true;
    }
    if keys.just_released(KeyCode::PageUp) && state.current >= 10 {
        state.current -= 10;
        changed = true;
    }
    if changed {
        update_title(&state, images, windows);
        for mut handle in query.iter_mut() {
            *handle = state.images[state.current].clone();
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ViewerState::default())
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(keyboard_input)
        .run();
}
