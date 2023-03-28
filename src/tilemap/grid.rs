use bracket_pathfinding::prelude::SmallVec;
use num_integer::Roots;

use super::urect::URect;

/// A 2D map of tiles.
#[derive(Debug, Clone)]
pub struct Grid<T> {
    /// The tiles in the map.
    pub(crate) tiles: Vec<T>,

    /// Width of the map in tiles.
    pub width: usize,

    /// Height of the map in tiles.
    pub height: usize,
}

#[derive(Debug, Copy, Clone)]
pub enum LineDirection {
    Vertical,
    Horizontal,
    Heuristic,
}

// TODO: I think I'd like if this code used u32 instead of usize. Orr maybe i32? I dunno.
// With iRect I'd be able to go negative, which would be nice.
impl<T: Default + Clone> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            tiles: vec![T::default(); width * height],
            width,
            height,
        }
    }

    /// Returns the tile at the given coordinates.
    pub fn get_tile(&self, x: usize, y: usize) -> &T {
        &self.tiles[y * self.width + x]
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

    /// Sets the tile at the given coordinates.
    pub fn set_tile(&mut self, x: usize, y: usize, tile: &T) {
        self.tiles[y * self.width + x] = tile.clone();
    }

    /// Applies a rectangle of a given tile type to the map.
    pub fn apply_rect_to_map(&mut self, rect: &URect, tile: &T) {
        for x in rect.x..rect.x + rect.width {
            for y in rect.y..rect.y + rect.height {
                self.set_tile(x, y, tile);
            }
        }
    }

    /// Applies a rectangle border or outline of a given tile type to the map.
    pub fn apply_rect_border_to_map(&mut self, rect: &URect, tile: &T) {
        for x in rect.x..rect.x + rect.width {
            self.set_tile(x, rect.y, tile);
            self.set_tile(x, rect.y + rect.height - 1, tile);
        }

        for y in rect.y..rect.y + rect.height {
            self.set_tile(rect.x, y, tile);
            self.set_tile(rect.x + rect.width - 1, y, tile);
        }
    }

    // TODO: This is filled. We need an outline version.
    pub fn apply_circle_to_map(&mut self, center: &(usize, usize), radius: usize, tile: &T) {
        for x in 0..self.width {
            for y in 0..self.height {
                let distance = ((x as i32 - center.0 as i32).pow(2)
                    + (y as i32 - center.1 as i32).pow(2))
                .sqrt() as usize;

                if distance <= radius {
                    self.set_tile(x, y, tile);
                }
            }
        }
    }

    /// Applies a border of a given tile type to the map.
    pub fn apply_border_to_map(&mut self, tile: &T) {
        for x in 0..self.width {
            self.set_tile(x, 0, tile);
            self.set_tile(x, self.height - 1, tile);
        }

        for y in 0..self.height {
            self.set_tile(0, y, tile);
            self.set_tile(self.width - 1, y, tile);
        }
    }

    /// Applies a line of a given tile type to the map between the specified start
    /// and end points.
    ///
    /// This is a modified version of Bresenham's line algorithm.
    pub fn apply_line_to_map(&mut self, start: (usize, usize), end: (usize, usize), tile: &T) {
        // initialize the current position to the starting position
        let (mut current_row, mut current_col) = start;
        let (end_row, end_col) = end;

        // calculate the absolute difference between the starting and ending positions
        let delta_row = (end_row as i32 - current_row as i32).abs();
        let delta_col = (end_col as i32 - current_col as i32).abs();

        // determine the direction to step in both dimensions. For example
        // if the starting position is (0, 0) and the ending position is (2, 2),
        // the step in the row dimension will be 1 and the step in the column
        // dimension will be 1. If the starting position is (2, 2) and the
        // ending position is (0, 0), the step in the row dimension will be -1
        // and the step in the column dimension will be -1.
        let step_row = if current_row < end_row { 1 } else { -1 };
        let step_col = if current_col < end_col { 1 } else { -1 };

        // initialize the error term to the negative column difference
        let mut error = delta_row - delta_col;

        loop {
            // set the tile at the current position to the specified tile type
            self.set_tile(current_row, current_col, tile);

            if current_row == end_row && current_col == end_col {
                // if we've reached the end point, break out of the loop
                break;
            }

            // calculate the error term multiplied by 2. We multiply by 2
            // because we're going to add it to the error term in the column
            // dimension and subtract it from the error term in the row
            // dimension. Multiplying by 2 allows us to avoid a division
            // operation.
            let double_error = 2 * error;

            if double_error > -delta_col {
                // if the error term in the column dimension is greater than the
                // negative column difference, update the row position and adjust
                // the error term.
                error -= delta_col;
                current_row = (current_row as i32 + step_row) as usize;
            }

            if double_error < delta_row {
                // if the error term in the row dimension is less than the row
                // difference, update the column position and adjust the error term
                error += delta_row;
                current_col = (current_col as i32 + step_col) as usize;
            }
        }
    }

    /// Draws a line between the start and end points, using the provided tile type.
    /// The line will start at `start` and then move either vertically or
    /// horizontally until it on the same row or column as `end`. Then it will move
    /// either horizontally or vertically until it reaches `end`.
    pub fn draw_line_with_bend(
        &mut self,
        start: (usize, usize),
        end: (usize, usize),
        tile: &T,
        line_direction: LineDirection,
    ) {
        let (start_row, start_col) = start;
        let (end_row, end_col) = end;

        match line_direction {
            LineDirection::Horizontal => {
                self.apply_line_to_map(start, (start_row, end_col), tile);
                self.apply_line_to_map((start_row, end_col), end, tile);
            }
            LineDirection::Vertical => {
                self.apply_line_to_map(start, (end_row, start_col), tile);
                self.apply_line_to_map((end_row, start_col), end, tile);
            }
            _ => todo!(),
        }
    }
}
