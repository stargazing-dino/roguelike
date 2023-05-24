use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use constants::{MAP_SIZE, TILE_SIZE};
use input::player_action::PlayerAction;
use leafwing_input_manager::InputManagerBundle;
use movement::{move_ability::MoveAbility, wander_action::WanderAction};
use plugins::{
    enemies::UnitsPlugin,
    graphics::GraphicsPlugin,
    player::{Player, PlayerBundle, PlayerPlugin},
};
use rand::seq::SliceRandom;
use tilemap::{
    entities_from_tilemap::entities_from_tilemap, generate_tilemap::generate_tilemap,
    tile_type::TileType,
};

use vision::{explored_tiles::ExploredTiles, viewshed::Viewshed};

mod basic_needs;
mod behavior;
mod combat;
mod constants;
mod graphics;
mod input;
mod interaction;
mod movement;
mod plugins;
mod tilemap;
mod vision;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

// TODO: Move this to a resource or something... I think
#[derive(Component)]
pub struct Map {}

fn main() {
    App::new()
        .add_plugin(GraphicsPlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(UnitsPlugin)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup)
        .run();
}

// TODO: Move this code into plugins.

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load("colored_packed.png");
    let grid_size = TILE_SIZE.into();
    let map_type = TilemapType::default();

    // Background layer
    let mut tile_storage = TileStorage::empty(MAP_SIZE);
    let tilemap_entity = commands.spawn_empty().id();
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
            MoveAbility {
                last_moved: 0.0,
                speed: 0.5,
            },
            Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
            },
            ExploredTiles::default(),
        ))
        .id();

    tile_storage.set(&player_position, player_entity);

    for _ in 0..3 {
        let starting_monster_position = safe_spawn_tiles.choose(&mut rng).unwrap();
        let monster_position = TilePos {
            x: starting_monster_position.0 as u32,
            y: starting_monster_position.1 as u32,
        };

        let monster_entity = commands
            .spawn((
                TileBundle {
                    position: monster_position,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileType::HumanWithHoodedCowl.index(),
                    ..Default::default()
                },
                WanderAction,
                MoveAbility {
                    last_moved: 0.0,
                    speed: 0.5,
                },
                Viewshed {
                    visible_tiles: Vec::new(),
                    range: 8,
                },
            ))
            .id();

        tile_storage.set(&monster_position, monster_entity);
    }

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
