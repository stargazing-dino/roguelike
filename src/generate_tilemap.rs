use bevy_ecs_tilemap::prelude::*;
use rand::Rng;

use crate::{
    constants::TileType,
    tilemap::{urect::URect, LineDirection, Tilemap},
};

pub fn generate_tilemap(size: TilemapSize) -> (Tilemap, (usize, usize)) {
    let mut tilemap = Tilemap::new(size.x as usize, size.y as usize);
    let rng = &mut rand::thread_rng();

    let mut rooms: Vec<URect> = Vec::new();

    const MAX_ROOMS: i32 = 10;
    const MIN_SIZE: i32 = 4;
    const MAX_SIZE: i32 = 10;

    for _ in 0..MAX_ROOMS {
        let w = rng.gen_range(MIN_SIZE..=MAX_SIZE) as usize;
        let h = rng.gen_range(MIN_SIZE..=MAX_SIZE) as usize;
        let x = rng.gen_range(1..size.x as usize - w - 1);
        let y = rng.gen_range(1..size.y as usize - h - 1);
        let new_room = URect::new(x, y, w, h);
        let mut ok = true;

        for other_room in rooms.iter() {
            if new_room.intersect(other_room) {
                ok = false
            }
        }

        if ok {
            tilemap.apply_circle_to_map(&new_room.center(), 3, TileType::Cactus);
            // tilemap.apply_rect_border_to_map(&new_room, TileType::Cactus);

            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                tilemap.draw_line_with_bend(
                    (new_x, new_y),
                    (prev_x, prev_y),
                    TileType::GroundPathLarge,
                    LineDirection::Vertical,
                );
            }

            rooms.push(new_room);
        }
    }

    tilemap.apply_border_to_map(TileType::GroundPathLarge);

    // (tilemap, rooms[0].center())
    (tilemap, (10, 10))
}
