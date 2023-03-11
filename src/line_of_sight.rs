use bevy::prelude::Entity;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};

pub trait Visibility {
    fn is_visible<F>(&self, from: &TilePos, to: &TilePos, test_func: F) -> bool
    where
        F: Fn(&Entity, TilePos) -> bool;
}

// TODO: This uses the same bresenham algorithm as the movement system. We should probably
// refactor this into a common module.
impl Visibility for TileStorage {
    fn is_visible<F>(&self, from: &TilePos, to: &TilePos, test_func: F) -> bool
    where
        F: Fn(&Entity, TilePos) -> bool,
    {
        let mut x1 = from.x as i32;
        let mut y1 = from.y as i32;
        let x2 = to.x as i32;
        let y2 = to.y as i32;
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let mut err = dx - dy;
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        while x1 != x2 || y1 != y2 {
            let entity = self
                .get(&TilePos {
                    x: x1 as u32,
                    y: y1 as u32,
                })
                .unwrap();

            if !test_func(
                &entity,
                TilePos {
                    x: x1 as u32,
                    y: y1 as u32,
                },
            ) {
                return false;
            }
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x1 += sx;
            }
            if e2 < dx {
                err += dx;
                y1 += sy;
            }
        }
        true
    }
}
