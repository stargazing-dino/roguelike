use bevy::prelude::Component;
use bevy_ecs_tilemap::tiles::TilePos;

/// A list of tiles that have been explored by the player
#[derive(Default, Component, Debug, Clone)]
pub struct ExploredTiles(pub Vec<TilePos>);
