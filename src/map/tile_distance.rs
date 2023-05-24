use bevy_ecs_tilemap::tiles::TilePos;
use num_integer::Roots;

pub trait TileDistance {
    fn distance(&self, other: &Self) -> u32;
}

impl TileDistance for TilePos {
    fn distance(&self, other: &Self) -> u32 {
        let x = (self.x as i32 - other.x as i32).abs();
        let y = (self.y as i32 - other.y as i32).abs();

        (x.pow(2) + y.pow(2)).sqrt() as u32
    }
}
