use bevy::prelude::Component;
use bevy_ecs_tilemap::tiles::TilePos;

#[derive(Component)]
pub struct Viewshed {
    pub range: u32,
    pub visible_tiles: Vec<TilePos>,
}

impl Default for Viewshed {
    fn default() -> Self {
        Self {
            range: 8,
            visible_tiles: Vec::new(),
        }
    }
}
