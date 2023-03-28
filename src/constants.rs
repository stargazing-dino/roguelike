use bevy_ecs_tilemap::prelude::{TilemapSize, TilemapTileSize};

// I'd like if this was dynamic based on screen size? Maybe. I dunno.
pub const MAP_SIZE: TilemapSize = TilemapSize { x: 24, y: 42 };

pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 16.0, y: 16.0 };
