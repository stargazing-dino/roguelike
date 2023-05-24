use num_integer::Roots;

use super::urect::URect;

#[derive(Debug, Copy, Clone)]
pub enum LineDirection {
    Vertical,
    Horizontal,
    Heuristic,
}

// TODO: Rename. This doesn't have anything to do with rendering which
// is what a canvas makes me think of.

pub trait Grid2D<T> {
    fn set(&mut self, x: usize, y: usize, item: &T);

    fn get(&self, x: usize, y: usize) -> &T;

    fn width(&self) -> usize;

    fn height(&self) -> usize;

    /// Applies a rectangle of a given tile type to the map.
    fn apply_rect(&mut self, rect: &URect, item: &T) {
        for x in rect.x..rect.x + rect.width {
            for y in rect.y..rect.y + rect.height {
                self.set(x, y, item);
            }
        }
    }

    /// Applies a rectangle border or outline of a given tile type to the map.
    fn apply_rect_border(&mut self, rect: &URect, item: &T) {
        for x in rect.x..rect.x + rect.width {
            self.set(x, rect.y, item);
            self.set(x, rect.y + rect.height - 1, item);
        }

        for y in rect.y..rect.y + rect.height {
            self.set(rect.x, y, item);
            self.set(rect.x + rect.width - 1, y, item);
        }
    }

    // TODO: This is filled. We need an outline version.
    fn apply_circle(&mut self, center: &(usize, usize), radius: usize, item: &T) {
        for x in 0..self.width() {
            for y in 0..self.height() {
                let distance = ((x as i32 - center.0 as i32).pow(2)
                    + (y as i32 - center.1 as i32).pow(2))
                .sqrt() as usize;

                if distance <= radius {
                    self.set(x, y, item);
                }
            }
        }
    }

    /// Applies a line of a given tile type to the map between the specified start
    /// and end points.
    ///
    /// This is a modified version of Bresenham's line algorithm.
    fn apply_line(&mut self, start: (usize, usize), end: (usize, usize), item: &T) {
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
            self.set(current_row, current_col, item);

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

    /// Applies a border of a given tile type to the map.
    fn apply_border(&mut self, item: &T) {
        for x in 0..self.width() {
            self.set(x, 0, item);
            self.set(x, self.height() - 1, item);
        }

        for y in 0..self.height() {
            self.set(0, y, item);
            self.set(self.width() - 1, y, item);
        }
    }

    /// Draws a line between the start and end points, using the provided tile type.
    /// The line will start at `start` and then move either vertically or
    /// horizontally until it on the same row or column as `end`. Then it will move
    /// either horizontally or vertically until it reaches `end`.
    fn apply_line_with_bend(
        &mut self,
        start: (usize, usize),
        end: (usize, usize),
        item: &T,
        line_direction: LineDirection,
    ) {
        let (start_row, start_col) = start;
        let (end_row, end_col) = end;

        match line_direction {
            LineDirection::Horizontal => {
                self.apply_line(start, (start_row, end_col), item);
                self.apply_line((start_row, end_col), end, item);
            }
            LineDirection::Vertical => {
                self.apply_line(start, (end_row, start_col), item);
                self.apply_line((end_row, start_col), end, item);
            }
            _ => todo!(),
        }
    }
}
