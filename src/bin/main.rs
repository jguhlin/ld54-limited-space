use bevy::prelude::*;
use bevy_rand::prelude::*;
use rand_core::RngCore;
use bevy_prng::*;
use xxhash_rust::const_xxh3::xxh3_64 as const_xxh3;
use xxhash_rust::xxh3::xxh3_64;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use ld54gamelib::*;

// https://ludumdare.com/resources/guides/embedding/

fn main() {

    let seed: [u8; 16] = "IJNM543980uadsfa".as_bytes().try_into().expect("Incorrect length");

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
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    // text
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0., 0.47, 1.),
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        ..default()
    });
}
