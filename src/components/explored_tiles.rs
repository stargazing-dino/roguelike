use bevy::prelude::Component;
use bevy_ecs_tilemap::tiles::TilePos;

#[derive(Default, Component)]
pub struct ExploredTiles(pub Vec<TilePos>);
