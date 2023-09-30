use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_prng::*;
use bevy_rand::prelude::*;
use noise::{NoiseFn, Perlin, PerlinSurflet, Seedable};
use rand::{thread_rng, Rng};
use rand_core::RngCore;

use crate::settings::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::BuildMap), setup_map)
        .add_plugins(TilemapPlugin)
        .add_systems(Update, movement.run_if(in_state(GameState::BuildMap)))
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
    let map_size = TilemapSize { x: 40, y: 40 };
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    let open_width = rng.gen_range(4..9);
    let open_height = rng.gen_range(4..9);

    let mut map: Vec<u32> = vec![0; (map_size.x * map_size.y) as usize];

    // Start at the 10 x 10 area at the bottom right (So last 100 tiles)

    let map_x = map_size.x as usize;
    let map_y = map_size.y as usize;

    let start_x = 10 - open_width / 2;
    let start_y = 10 - open_height / 2;

    println!("Open Width: {} Open Height: {}", open_width, open_height);
    println!("Start {} {}", start_x, start_y);

    for x in 0..open_width {
        for y in 0..open_height {
            println!("{} {}", x, y);
            map[(start_x + x) * map_x + start_y + y] = 2;
        }
    }

    // Resources
    let total_space = open_height * open_width;
    let half_space = total_space / 2;
    let resources = rng.gen_range(2..half_space);


    // Spawn the elements of the tilemap.
    // Alternatively, you can use helpers::filling::fill_tilemap.
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(map[x as usize * map_x + y as usize]),
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

#[allow(dead_code)]
pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    for (mut transform, mut ortho) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Z) {
            ortho.scale += 0.1;
        }

        if keyboard_input.pressed(KeyCode::X) {
            ortho.scale -= 0.1;
        }

        if ortho.scale < 0.5 {
            ortho.scale = 0.5;
        }

        let z = transform.translation.z;
        transform.translation += time.delta_seconds() * direction * 500.;
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }
}
