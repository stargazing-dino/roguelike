use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use components::playable::Playable;
use constants::{MAP_SIZE, TILE_SIZE};
use map::fill_tilemap_and_walls;
use systems::move_playable::move_playable;

mod camera;
mod components;
mod constants;
mod map;
mod systems;

// TODO: Move this to a resource or something... I think
#[derive(Component)]
pub struct Map {}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: String::from("Update tile positions without despawning."),
                        height: 720.0,
                        width: 460.0,
                        ..Default::default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(TilemapPlugin)
        .add_startup_system(setup)
        .add_system(camera::movement)
        .add_system(move_playable)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // 49 x 22
    let texture_handle: Handle<Image> = asset_server.load("colored_packed.png");
    let grid_size = TILE_SIZE.into();
    let map_type = TilemapType::default();

    // Background layer
    let mut tile_storage = TileStorage::empty(MAP_SIZE);
    let tilemap_entity = commands.spawn_empty().id();
    let player_pos = TilePos { x: 16, y: 16 };

    fill_tilemap_and_walls(
        TileTextureIndex(1),
        TileTextureIndex(637),
        player_pos,
        MAP_SIZE,
        TilemapId(tilemap_entity),
        &mut commands,
        &mut tile_storage,
    );

    commands.entity(tilemap_entity).insert((
        TilemapBundle {
            grid_size,
            map_type,
            size: MAP_SIZE,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle.clone()),
            tile_size: TILE_SIZE,
            transform: get_tilemap_center_transform(&MAP_SIZE, &grid_size, &map_type, 0.0),
            ..Default::default()
        },
        Map {},
    ));

    // Foreground layer
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(MAP_SIZE);
    let tile_entity = commands
        .spawn((
            TileBundle {
                position: player_pos,
                tilemap_id: TilemapId(tilemap_entity),
                // 8 * 49 + 18 = 418
                texture_index: TileTextureIndex(416),
                ..Default::default()
            },
            Playable {},
        ))
        .id();

    tile_storage.set(&player_pos, tile_entity);

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: MAP_SIZE,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size: TILE_SIZE,
        transform: get_tilemap_center_transform(&MAP_SIZE, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}
