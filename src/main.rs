use bevy::prelude::*;

const TITLE: &str = concat!("Questra Alpha ", env!("CARGO_PKG_VERSION_PATCH"));

fn hi() {
    println!("HAI");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Update, hi)
        .run();
}

// fn advance_track(index: &mut usize, track_count: usize) {
//     *index = (*index + 1) % track_count
// }
