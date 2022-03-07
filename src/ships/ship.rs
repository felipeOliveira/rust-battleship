use super::{Coordinate, Orientation, ShipType};

#[derive(Debug, PartialEq)]
pub struct Ship {
    size: u8,
    ship_type: ShipType,
    coord: Coordinate,
    orientation: Orientation,
}

impl Ship {
    pub fn new(ship_type: ShipType, coord: Coordinate, orientation: Orientation) -> Ship {
        let ship_size = match ship_type {
            ShipType::AircraftCarrier => 5,
            ShipType::Battleship => 4,
            ShipType::Destroyer | ShipType::Submarine => 3,
            ShipType::PatrolBoat => 2,
        };

        Ship {
            size: ship_size,
            ship_type,
            coord,
            orientation,
        }
    }

    pub fn coord(&self) -> &Coordinate {
        &self.coord
    }

    pub fn orientation(&self) -> &Orientation {
        &self.orientation
    }

    pub fn size(&self) -> &u8 {
        &self.size
    }

    pub fn ship_type(&self) -> &ShipType {
        &self.ship_type
    }

    pub fn get_full_position(&self) -> (u8, u8, u8, u8) {
        let col = *self.coord.col();
        let row = *self.coord.row();
        
        match self.orientation {
            Orientation::Landscape => (col, col + self.size - 1, row, row),
            Orientation::Portrait => (col, col, row, row + self.size - 1),
        }
    }
}
