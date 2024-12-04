#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}

impl Coordinates {
    pub fn to_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

impl From<(usize, usize)> for Coordinates {
    fn from((x, y): (usize, usize)) -> Self {
        Coordinates { x, y }
    }
}
