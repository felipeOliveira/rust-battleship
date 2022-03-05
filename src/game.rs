use std::collections::HashMap;
use crate::ships::{Coordinate, Orientation, Ship, ShipType};

pub struct Game {
    columns: Vec<u8>,
    rows: Vec<u8>,
    player_ships: HashMap<&'static str, Vec<Ship>>,
}

impl Game {
    pub fn new(player1: &'static str, player2: &'static str) -> Game {
        let mut player_ships: HashMap<&str, Vec<Ship>> = HashMap::new();
        player_ships.insert(&player1, Vec::new());
        player_ships.insert(&player2, Vec::new());

        Game {
            columns: (1..=10).collect::<Vec<u8>>(),
            rows: (1..=10).collect::<Vec<u8>>(),
            player_ships,
        }
    }

    pub fn create_ship(
        &mut self,
        player: &str,
        ship_type: ShipType,
        coordinate: Coordinate,
        orientation: Orientation,
    ) -> Result<(), String> {
        if let Err(e) = self.ensure_is_valid_coords(&coordinate) {
            return Err(e);
        }

        let new_ship = Ship::new(ship_type, coordinate, orientation);
        if let Err(e) = self.ensure_is_not_overlaping(self.player_ships.get(player), &new_ship) {
            return Err(e);
        }

        if let Some(ships) = self.player_ships.get_mut(player) {
            ships.push(new_ship);
        } else {
            return Err("player not found".into());
        }

        Ok(())
    }

    pub fn player_ships(&self) -> &HashMap<&'static str, Vec<Ship>> {
        &self.player_ships
    }

    fn ensure_is_valid_coords(&self, ship_coords: &Coordinate) -> Result<(), String> {
        if !self.columns.contains(ship_coords.col()) {
            return Err("invalid column".into());
        }

        if !self.rows.contains(ship_coords.row()) {
            return Err("invalid row".into());
        }

        Ok(())
    }

    fn ensure_is_not_overlaping(
        &self,
        player_ships: Option<&Vec<Ship>>,
        new_ship: &Ship,
    ) -> Result<(), String> {
        if let None = player_ships {
            return Ok(());
        }

        let (start_col, end_col, start_row, end_row) = new_ship.get_full_position();

        for ship in player_ships.unwrap().into_iter() {
            let (ship_start_col, ship_end_col, ship_start_row, ship_end_row) =
                ship.get_full_position();

            if start_col >= ship_start_col && end_col <= ship_end_col
                || start_row >= ship_start_row && end_row <= ship_end_row
            {
                return Err("could not be possible posicioning the ship on this coords. This ship will overlap another ship".into());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_ship_by_type() {
        let mut game = Game::new("User1".into(), "Computer".into());

        let _ = game.create_ship(
            "User1",
            ShipType::AircraftCarrier,
            Coordinate::new(1, 1),
            Orientation::Portrait,
        );

        let expected_ship = Ship::new(
            ShipType::AircraftCarrier,
            Coordinate::new(1, 1),
            Orientation::Portrait,
        );

        let ships = game.player_ships().get("User1").unwrap();
        assert_eq!(1, ships.len());
        assert_eq!(expected_ship, ships[0]);
    }

    #[test]
    fn returns_error_when_column_is_invalid() {
        let mut game = Game::new("User1".into(), "Computer".into());

        let result = game.create_ship(
            "User1",
            ShipType::AircraftCarrier,
            Coordinate::new(11, 5),
            Orientation::Portrait,
        );

        let e: Result<(), String> = Err("invalid column".into());
        assert_eq!(e, result);
    }

    #[test]
    fn returns_error_when_row_is_invalid() {
        let mut game = Game::new("User1".into(), "Computer".into());

        let result = game.create_ship(
            "User1",
            ShipType::AircraftCarrier,
            Coordinate::new(10, 30),
            Orientation::Portrait,
        );

        let e: Result<(), String> = Err("invalid row".into());
        assert_eq!(e, result);
    }

    #[test]
    fn returns_error_when_a_ship_overlap_another_ship_on_vertical() {
        let mut game = Game::new("User1".into(), "Computer".into());

        let _ = game.create_ship(
            "User1",
            ShipType::AircraftCarrier,
            Coordinate::new(1, 5),
            Orientation::Portrait,
        );

        let e: Result<(), String> = Err("could not be possible posicioning the ship on this coords. This ship will overlap another ship".into());

        let result = game.create_ship(
            "User1",
            ShipType::PatrolBoat,
            Coordinate::new(1, 9),
            Orientation::Portrait,
        );

        assert_eq!(e, result);

        let result = game.create_ship(
            "User1",
            ShipType::Destroyer,
            Coordinate::new(1, 4),
            Orientation::Portrait,
        );

        assert_eq!(e, result);
    }

    #[test]
    fn returns_error_when_a_ship_overlap_another_ship_on_horizontal() {
        let mut game = Game::new("User1".into(), "Computer".into());

        let _ = game.create_ship(
            "User1",
            ShipType::AircraftCarrier,
            Coordinate::new(2, 2),
            Orientation::Landscape,
        );

        let e: Result<(), String> = Err("could not be possible posicioning the ship on this coords. This ship will overlap another ship".into());

        let result = game.create_ship(
            "User1",
            ShipType::Destroyer,
            Coordinate::new(1, 2),
            Orientation::Landscape,
        );
        assert_eq!(e, result);

        let result = game.create_ship(
            "User1",
            ShipType::Battleship,
            Coordinate::new(4, 2),
            Orientation::Landscape,
        );
        assert_eq!(e, result);

        let result = game.create_ship(
            "User1",
            ShipType::Submarine,
            Coordinate::new(6, 2),
            Orientation::Landscape,
        );
        assert_eq!(e, result);

        let result = game.create_ship(
            "User1",
            ShipType::PatrolBoat,
            Coordinate::new(4, 1),
            Orientation::Portrait,
        );
        assert_eq!(e, result);
    }
}
