use bracket_pathfinding::prelude::SmallVec;
use num_integer::Roots;

use super::urect::URect;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    /// The items in the grid
    pub(crate) items: Vec<T>,

    /// Width of the map in items.
    pub width: usize,

    /// Height of the map in items.
    pub height: usize,
}

// TODO: I think I'd like if this code used u32 instead of usize. Orr maybe i32? I dunno.
// With iRect I'd be able to go negative, which would be nice.
impl<T: Default + Clone> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            items: vec![T::default(); width * height],
            width,
            height,
        }
    }

    /// Returns the tile at the given coordinates.
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.items[y * self.width + x]
    }

    /// Sets the tile at the given coordinates.
    pub fn set(&mut self, x: usize, y: usize, item: &T) {
        self.items[y * self.width + x] = item.clone();
    }

    /// Returns the neighbors of the given coordinates.
    pub fn get_neighbors(&self, x: usize, y: usize) -> SmallVec<[(usize, usize); 8]> {
        let mut neighbors = SmallVec::new();

        if x > 0 {
            neighbors.push((x - 1, y));
        }

        if x < self.width - 1 {
            neighbors.push((x + 1, y));
        }

        if y > 0 {
            neighbors.push((x, y - 1));
        }

        if y < self.height - 1 {
            neighbors.push((x, y + 1));
        }

        neighbors
    }
}
