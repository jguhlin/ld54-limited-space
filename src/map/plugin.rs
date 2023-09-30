use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_prng::*;
use bevy_rand::prelude::*;
use noise::{NoiseFn, Perlin, PerlinSurflet, Seedable};
use rand_core::RngCore;

use crate::settings::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::BuildMap), setup_map)
        .add_plugins(TilemapPlugin)
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
    let texture_handle: Handle<Image> = asset_server.load("Tiles.png");
    let map_size = TilemapSize { x: 32, y: 32 };
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    // TODO: Use resource seed
    let perlin = PerlinSurflet::new(428);

    // Spawn the elements of the tilemap.
    // Alternatively, you can use helpers::filling::fill_tilemap.
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 64.0, y: 64.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}
