use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum ShipType {
    Battleship,
    PatrolBoat,
    Submarine,
    Destroyer,
    AircraftCarrier,
}

impl Display for ShipType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ShipType::Battleship => write!(f, "Battleship"),
            ShipType::PatrolBoat => write!(f, "Patrol Boat"),
            ShipType::Submarine => write!(f, "Submarine"),
            ShipType::Destroyer => write!(f, "Destroyer"),
            ShipType::AircraftCarrier => write!(f, "Aircraft Carrier"),
        }
    }
}
