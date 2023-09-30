use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prng::*;
use bevy_rand::prelude::*;
use rand_core::RngCore;
use xxhash_rust::const_xxh3::xxh3_64 as const_xxh3;
use xxhash_rust::xxh3::xxh3_64;

use ld54gamelib::map::MapPlugin;
use ld54gamelib::menu::MenuPlugin;
use ld54gamelib::*;

// https://ludumdare.com/resources/guides/embedding/

fn main() {
    // TODO: Let the seed be user definable...
    let seed: [u8; 16] = "IJNM543980uadsfa"
        .as_bytes()
        .try_into()
        .expect("Incorrect length");

    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: GAME_TITLE.to_string(), // ToDo
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                // Bind to canvas included in `index.html`
                // TODO: Will need this later...
                canvas: Some("#bevy".to_owned()),
                // Tells wasm not to override default event handling, like F5 and Ctrl+R
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EntropyPlugin::<Xoshiro128StarStar>::with_seed(seed))
        .add_plugins(WorldInspectorPlugin::new())
        .add_state::<GameState>()
        .insert_resource(Volume::default())
        .insert_resource(Seed::default())
        .add_plugins((MenuPlugin, MapPlugin))
        .add_systems(Startup, setup)
        // Run game
        .run();
}

// Setup just does camera now...
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
