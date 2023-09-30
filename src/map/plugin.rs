use bevy::prelude::*;
use noise::{NoiseFn, Perlin, Seedable, PerlinSurflet};
use bevy_prng::*;
use bevy_rand::prelude::*;
use rand_core::RngCore;
use bevy_ecs_tilemap::prelude::*;

use crate::settings::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::BuildMap), setup_map)
            /*.add_systems(
                Update,
                (menu, text_input).run_if(in_state(GameState::BuildMap)),
            )
            .add_systems(OnExit(GameState::BuildMap), cleanup_menu) */
            ;
    }
}

fn setup_map(
    mut commands: Commands,
    mut rng: ResMut<GlobalEntropy<Xoshiro128StarStar>>, // TODO: Use seeded Rng
    asset_server: Res<AssetServer>,
) {
    let texture_handle: Handle<Image> = asset_server.load("tiles.png");
    let map_size = TilemapSize { x: 32, y: 32 };
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    // TODO: Use resource seed
    let perlin = PerlinSurflet::new(428);


}
