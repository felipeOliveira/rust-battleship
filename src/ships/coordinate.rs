#[derive(Debug, PartialEq)]
pub struct Coordinate {
    col: u8,
    row: u8,
}

impl Coordinate {
    pub fn new(col: u8, row: u8) -> Self {
        Coordinate { col, row }
    }

    pub fn col(&self) -> &u8 {
        &self.col
    }

    pub fn row(&self) -> &u8 {
        &self.row
    }
}
