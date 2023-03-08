use rand::Rng;

use crate::constants::TileType;

// TODO: This might be a good candidate for a command buffer.
pub struct Tilemap {
    tiles: Vec<Vec<TileType>>,
}

impl Tilemap {
    pub fn new(width: usize, height: usize) -> Tilemap {
        Tilemap {
            tiles: vec![vec![TileType::Ground; width]; height],
        }
    }

    pub fn width(&self) -> usize {
        self.tiles[0].len()
    }

    pub fn height(&self) -> usize {
        self.tiles.len()
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &TileType {
        &self.tiles[y][x]
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: TileType) {
        self.tiles[y][x] = tile;
    }

    pub fn apply_rect_to_map(&mut self, rect: &URect, tile: TileType) {
        for x in rect.x..rect.x + rect.width {
            for y in rect.y..rect.y + rect.height {
                self.set_tile(x, y, tile);
            }
        }
    }

    pub fn apply_rect_border_to_map(&mut self, rect: &URect, tile: TileType) {
        for x in rect.x..rect.x + rect.width {
            self.set_tile(x, rect.y, tile);
            self.set_tile(x, rect.y + rect.height - 1, tile);
        }

        for y in rect.y..rect.y + rect.height {
            self.set_tile(rect.x, y, tile);
            self.set_tile(rect.x + rect.width - 1, y, tile);
        }
    }

    pub fn apply_border_to_map(&mut self, tile: TileType) {
        for x in 0..self.tiles[0].len() {
            self.set_tile(x, 0, tile);
            self.set_tile(x, self.tiles.len() - 1, tile);
        }

        for y in 0..self.tiles.len() {
            self.set_tile(0, y, tile);
            self.set_tile(self.tiles[0].len() - 1, y, tile);
        }
    }

    pub fn apply_line_to_map(
        &mut self,
        start: (usize, usize),
        end: (usize, usize),
        tile: TileType,
    ) {
        let (mut x, mut y) = start;
        let (end_x, end_y) = end;

        let dx = (end_x as i32 - x as i32).abs();
        let dy = (end_y as i32 - y as i32).abs();

        let sx = if x < end_x { 1 } else { -1 };
        let sy = if y < end_y { 1 } else { -1 };

        let mut err = dx - dy;

        loop {
            self.set_tile(x, y, tile);

            if x == end_x && y == end_y {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x = (x as i32 + sx) as usize;
            }
            if e2 < dx {
                err += dx;
                y = (y as i32 + sy) as usize;
            }
        }
    }
}

pub struct URect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl URect {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> URect {
        URect {
            x,
            y,
            width,
            height,
        }
    }

    pub fn center(&self) -> (usize, usize) {
        let center_x = self.x + self.width / 2;
        let center_y = self.y + self.height / 2;
        (center_x, center_y)
    }

    pub fn intersect(&self, other: &URect) -> bool {
        (self.x <= other.x + other.width)
            && (self.x + self.width >= other.x)
            && (self.y <= other.y + other.height)
            && (self.y + self.height >= other.y)
    }
}
