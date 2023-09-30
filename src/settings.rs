use bevy::prelude::*;

// As per https://ludumdare.com/resources/guides/embedding/
pub const WINDOW_WIDTH: f32 = 948.0;
pub const WINDOW_HEIGHT: f32 = 533.0;

pub const GAME_TITLE: &str = "LDJam54";

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    Splash, // maybe
    Menu,
    #[default]
    BuildMap,
    InGame,
    ScoreScreen,
}

// UI Stuff
pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

// User Configurable Settings
#[derive(Resource)]
pub struct Volume(u8);

impl Default for Volume {
    fn default() -> Self {
        Volume(10)
    }
}

#[derive(Resource)]
pub struct Seed {
    pub raw: String,
    pub seed: [u8; 16],
}

impl Default for Seed {
    fn default() -> Self {
        Seed {
            raw: "Limited Space".into(),
            seed: default(),
        }
    }
}
