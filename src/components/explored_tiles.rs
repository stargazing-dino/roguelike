use bevy::prelude::Component;
use bevy_ecs_tilemap::tiles::TilePos;

#[derive(Default, Component, Debug, Clone)]
pub struct ExploredTiles(pub Vec<TilePos>);
