use bevy::{prelude::*, window::WindowResolution};
use bevy_ecs_tilemap::prelude::*;
use components::{
    explored_tiles::ExploredTiles,
    last_moved_time::LastMovedTime,
    player::{Player, PlayerBundle},
};
use constants::{TileType, MAP_SIZE, TILE_SIZE};
use input::players::PlayerAction;
use leafwing_input_manager::{prelude::InputManagerPlugin, InputManagerBundle};
use rand::seq::SliceRandom;
use systems::{
    explore_tiles::explore_tiles, move_playable::move_player,
    viewable_tiles_for_player::viewable_tiles_for_player, viewshed::update_viewshed,
};

use components::viewshed::Viewshed;
use thinkers::ThinkersPlugin;
use tilemap::{entities_from_tilemap::entities_from_tilemap, generate_tilemap::generate_tilemap};

mod actions;
mod camera;
mod components;
mod constants;
mod input;
mod line_of_sight;
mod scorers;
mod systems;
mod thinkers;
mod tilemap;

// TODO: Move this to a resource or something... I think
#[derive(Component)]
pub struct Map {}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Roguelike"),
                        resolution: WindowResolution::new(460., 720.),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(InputManagerPlugin::<PlayerAction>::default())
        .add_plugin(TilemapPlugin)
        .add_plugin(ThinkersPlugin)
        .add_startup_system(setup)
        .add_system(camera::movement)
        .add_system(explore_tiles)
        .add_system(move_player)
        .add_system(update_viewshed.after(move_player))
        .add_system(viewable_tiles_for_player.after(update_viewshed))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle: Handle<Image> = asset_server.load("colored_packed.png");
    let grid_size = TILE_SIZE.into();
    let map_type = TilemapType::default();

    // Background layer
    let mut tile_storage = TileStorage::empty(MAP_SIZE);
    let tilemap_entity = commands.spawn_empty().id();
    // TODO: Just return a tilemap and then have an impl that gives you a spot to place
    // the player.
    let tilemap = generate_tilemap(MAP_SIZE);

    entities_from_tilemap(
        &tilemap,
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
    let mut rng = rand::thread_rng();
    let safe_spawn_tiles = tilemap.get_safe_spawn_tiles();
    let starting_position = safe_spawn_tiles.choose(&mut rng).unwrap();
    let player_position = TilePos {
        x: starting_position.0 as u32,
        y: starting_position.1 as u32,
    };
    let player_entity = commands
        .spawn((
            TileBundle {
                position: player_position,
                tilemap_id: TilemapId(tilemap_entity),
                texture_index: TileType::Human.index(),
                ..Default::default()
            },
            PlayerBundle {
                player: Player::One,
                input_manager: InputManagerBundle {
                    input_map: PlayerAction::input_map_for(Player::One),
                    ..Default::default()
                },
            },
            LastMovedTime(0.0),
            Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
            },
            ExploredTiles::default(),
        ))
        .id();

    tile_storage.set(&player_position, player_entity);

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: MAP_SIZE,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size: TILE_SIZE,
        transform: get_tilemap_center_transform(&MAP_SIZE, &grid_size, &map_type, 100.0),
        ..Default::default()
    });
}
