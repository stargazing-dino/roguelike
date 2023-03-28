use bracket_pathfinding::prelude::DistanceAlg::Pythagoras;
use bracket_pathfinding::prelude::{Algorithm2D, BaseMap, Point, SmallVec};

use self::grid::Grid;
use self::tile_type::TileType;

pub mod entities_from_tilemap;
pub mod generate_tilemap;
pub mod grid;
pub mod tile_type;
pub mod urect;

type Tilemap = Grid<TileType>;

impl Grid<TileType> {
    fn valid_exit(&self, location: Point, delta: Point) -> Option<usize> {
        let dest = location + delta;
        if self.in_bounds(dest) {
            let index = self.point2d_to_index(dest);
            if !self.is_opaque(index) {
                Some(index)
            } else {
                None
            }
        } else {
            None
        }
    }

    // /// Returns a list of tiles that would be good candidates to spawn at.
    // /// This is determined by being a ground tile whose neighbors are all ground tiles.
    pub fn get_safe_spawn_tiles(&self) -> Vec<(usize, usize)> {
        let mut safe_spawn_tiles = Vec::new();

        for x in 0..self.width {
            for y in 0..self.height {
                let current_tile = self.get_tile(x, y);

                if current_tile.is_walkable() {
                    let mut is_safe = true;

                    for (nx, ny) in self.get_neighbors(x, y) {
                        let neighbor_tile = self.get_tile(nx, ny);

                        if !neighbor_tile.is_walkable() {
                            is_safe = false;
                        }
                    }

                    if is_safe {
                        safe_spawn_tiles.push((x, y));
                    }
                }
            }
        }

        safe_spawn_tiles
    }
}

impl Algorithm2D for Grid<TileType> {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Grid<TileType> {
    // Anything that is not walkable is opaque
    fn is_opaque(&self, _idx: usize) -> bool {
        !self.tiles[_idx].is_walkable()
    }

    fn get_available_exits(&self, _idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(_idx);

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(-1, -1)) {
            exits.push((idx, 1.4))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 1)) {
            exits.push((idx, 1.4))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, -1)) {
            exits.push((idx, 1.4))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 1)) {
            exits.push((idx, 1.4))
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}
