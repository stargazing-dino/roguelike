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
