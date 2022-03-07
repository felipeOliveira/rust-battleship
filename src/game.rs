use crate::{
    player::Player,
    ships::{Coordinate, Orientation, Ship, ShipType},
    turn::Turn,
};
use std::{collections::HashMap, fmt::Display};

pub struct Game<'a> {
    columns: Vec<u8>,
    rows: Vec<u8>,
    player_ships: HashMap<&'a str, Vec<Ship>>,
    player1: &'a Player,
    player2: &'a Player,
    turn: Turn<'a>,
}

impl<'a> Game<'a> {
    pub fn new(player1: &'a Player, player2: &'a Player) -> Game<'a> {
        Game {
            columns: (1..=10).collect::<Vec<u8>>(),
            rows: (1..=10).collect::<Vec<u8>>(),
            player_ships: HashMap::from([
                (player1.name(), Vec::new()),
                (player2.name(), Vec::new()),
            ]),
            player1,
            player2,
            turn: Turn::new(),
        }
    }

    pub fn create_ship(
        &mut self,
        player: &Player,
        ship_type: ShipType,
        coordinate: Coordinate,
        orientation: Orientation,
    ) -> Result<(), String> {
        if let Err(e) = {
            let ref this = self;
            let ship_coords = &coordinate;
            if !this.columns.contains(ship_coords.col()) {
                return Err("invalid column".into());
            }

            if !this.rows.contains(ship_coords.row()) {
                return Err("invalid row".into());
            }

            Ok(())
        } {
            return Err(e);
        }

        let new_ship = Ship::new(ship_type, coordinate, orientation);
        if let Err(e) = {
            let player_ships = self.player_ships.get(player.name());
            let new_ship = &new_ship;
            if let None = player_ships {
                return Ok(());
            }

            let (start_col, end_col, start_row, end_row) = new_ship.get_full_position();

            for ship in player_ships.unwrap().into_iter() {
                let (ship_start_col, ship_end_col, ship_start_row, ship_end_row) =
                    ship.get_full_position();

                if *new_ship.ship_type() == *ship.ship_type() {
                    return Err(
                        format!("you already have an {} on board", new_ship.ship_type()).into(),
                    );
                }

                if (start_col >= ship_start_col && start_col <= ship_end_col
                    || end_col >= ship_start_col && end_col <= ship_end_col)
                    && (start_row >= ship_start_row && start_row <= ship_end_row
                        || end_row >= ship_start_row && end_row <= ship_end_row)
                {
                    return Err("could not be possible posicioning the ship on this coords. This ship will overlap another ship".into());
                }
            }
            Ok(())
        } {
            return Err(e);
        }

        if let Some(ships) = self.player_ships.get_mut(player.name()) {
            ships.push(new_ship);
        } else {
            return Err("player not found".into());
        }

        Ok(())
    }

    pub fn player_ships(&self) -> &HashMap<&'a str, Vec<Ship>> {
        &self.player_ships
    }

    pub fn start(&mut self) -> Result<(), String> {
        if self.player_ships.get(self.player1.name()).unwrap().len() < 5 {
            return Err(
                format!("{} need positioning all ships before start", &self.player1).into(),
            );
        }

        if self.player_ships.get(self.player2.name()).unwrap().len() < 5 {
            return Err(
                format!("{} need positioning all ships before start", &self.player2).into(),
            );
        }

        self.turn.change(self.player1);

        Ok(())
    }

    pub fn round(&self) -> &usize {
        &self.turn.round()
    }

    pub fn player_turn(&self) -> Option<&'a Player> {
        self.turn.player()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_ship_by_type() {
        let player1 = Player::new(String::from("User1"));
        let player2 = Player::new(String::from("Computer"));

        let mut game = Game::new(&player1, &player2);

        let _ = game.create_ship(
            &player1,
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
        let player1 = Player::new(String::from("User1"));
        let player2 = Player::new(String::from("Computer"));

        let mut game = Game::new(&player1, &player2);

        let result = game.create_ship(
            &player1,
            ShipType::AircraftCarrier,
            Coordinate::new(11, 5),
            Orientation::Portrait,
        );

        let e: Result<(), String> = Err("invalid column".into());
        assert_eq!(e, result);
    }

    #[test]
    fn returns_error_when_row_is_invalid() {
        let player1 = Player::new(String::from("User1"));
        let player2 = Player::new(String::from("Computer"));

        let mut game = Game::new(&player1, &player2);

        let result = game.create_ship(
            &player1,
            ShipType::AircraftCarrier,
            Coordinate::new(10, 30),
            Orientation::Portrait,
        );

        let e: Result<(), String> = Err("invalid row".into());
        assert_eq!(e, result);
    }

    #[test]
    fn returns_error_when_a_ship_overlap_another_ship_on_vertical() {
        let player1 = Player::new(String::from("User1"));
        let player2 = Player::new(String::from("Computer"));

        let mut game = Game::new(&player1, &player2);

        let _ = game.create_ship(
            &player1,
            ShipType::AircraftCarrier,
            Coordinate::new(1, 5),
            Orientation::Portrait,
        );

        let e: Result<(), String> = Err("could not be possible posicioning the ship on this coords. This ship will overlap another ship".into());

        let result = game.create_ship(
            &player1,
            ShipType::PatrolBoat,
            Coordinate::new(1, 9),
            Orientation::Portrait,
        );

        assert_eq!(e, result);

        let result = game.create_ship(
            &player1,
            ShipType::Destroyer,
            Coordinate::new(1, 4),
            Orientation::Portrait,
        );

        assert_eq!(e, result);
    }

    #[test]
    fn returns_error_when_a_ship_overlap_another_ship_on_horizontal() {
        let player1 = Player::new(String::from("User1"));
        let player2 = Player::new(String::from("Computer"));

        let mut game = Game::new(&player1, &player2);

        let _ = game.create_ship(
            &player1,
            ShipType::AircraftCarrier,
            Coordinate::new(2, 2),
            Orientation::Landscape,
        );

        let e: Result<(), String> = Err("could not be possible posicioning the ship on this coords. This ship will overlap another ship".into());

        let result = game.create_ship(
            &player1,
            ShipType::Destroyer,
            Coordinate::new(1, 2),
            Orientation::Landscape,
        );
        assert_eq!(e, result);

        let result = game.create_ship(
            &player1,
            ShipType::Battleship,
            Coordinate::new(4, 2),
            Orientation::Landscape,
        );
        assert_eq!(e, result);

        let result = game.create_ship(
            &player1,
            ShipType::Submarine,
            Coordinate::new(6, 2),
            Orientation::Landscape,
        );
        assert_eq!(e, result);

        let result = game.create_ship(
            &player1,
            ShipType::PatrolBoat,
            Coordinate::new(4, 1),
            Orientation::Portrait,
        );
        assert_eq!(e, result);
    }

    #[test]
    fn should_returns_error_if_player_try_two_ship_of_same_type() {
        let player1 = Player::new(String::from("User1"));
        let player2 = Player::new(String::from("Computer"));

        let mut game = Game::new(&player1, &player2);

        let _ = game.create_ship(
            &player1,
            ShipType::AircraftCarrier,
            Coordinate::new(2, 2),
            Orientation::Landscape,
        );

        let e: Result<(), String> = Err("you already have an Destroyer on board".into());

        let _ = game.create_ship(
            &player1,
            ShipType::Destroyer,
            Coordinate::new(3, 5),
            Orientation::Landscape,
        );

        let result = game.create_ship(
            &player1,
            ShipType::Destroyer,
            Coordinate::new(4, 5),
            Orientation::Landscape,
        );
        assert_eq!(e, result);
    }

    #[test]
    fn should_returns_error_if_not_all_player1_ships_is_positioned() {
        let player1 = Player::new(String::from("User1"));
        let player2 = Player::new(String::from("Computer"));

        let mut game = Game::new(&player1, &player2);

        let result = game.start();

        assert_eq!(
            Err(String::from(
                "User1 need positioning all ships before start"
            )),
            result
        );
    }

    #[test]
    fn should_returns_error_if_not_all_player2_ships_is_positioned() {
        let player1 = Player::new(String::from("User1"));
        let player2 = Player::new(String::from("Computer"));

        let mut game = Game::new(&player1, &player2);

        let _ = game.create_ship(
            &player1,
            ShipType::AircraftCarrier,
            Coordinate::new(2, 1),
            Orientation::Landscape,
        );

        let _ = game.create_ship(
            &player1,
            ShipType::Battleship,
            Coordinate::new(8, 1),
            Orientation::Portrait,
        );

        let _ = game.create_ship(
            &player1,
            ShipType::Destroyer,
            Coordinate::new(3, 8),
            Orientation::Portrait,
        );

        let _ = game.create_ship(
            &player1,
            ShipType::PatrolBoat,
            Coordinate::new(5, 5),
            Orientation::Landscape,
        );

        let _ = game.create_ship(
            &player1,
            ShipType::Submarine,
            Coordinate::new(8, 7),
            Orientation::Landscape,
        );

        let result = game.start();

        assert_eq!(
            Err(String::from(
                "Computer need positioning all ships before start"
            )),
            result
        );
    }

    #[test]
    fn should_start_game() {
        let player1 = Player::new(String::from("User1"));
        let player2 = Player::new(String::from("Computer"));

        let mut game = create_game_with_ship(&player1, &player2);
        let result = game.start();

        assert!(result.is_ok());
        assert_eq!(*game.player_turn().unwrap(), player1);
        assert_eq!(*game.round(), 0);
    }

    fn create_game_with_ship<'a>(player1: &'a Player, player2: &'a Player) -> Game<'a> {
        let mut game = Game::new(&player1, &player2);

        let _ = game.create_ship(
            &player1,
            ShipType::AircraftCarrier,
            Coordinate::new(2, 1),
            Orientation::Landscape,
        );

        let _ = game.create_ship(
            &player1,
            ShipType::Battleship,
            Coordinate::new(8, 1),
            Orientation::Portrait,
        );

        let _ = game.create_ship(
            &player1,
            ShipType::Destroyer,
            Coordinate::new(3, 8),
            Orientation::Portrait,
        );

        let _ = game.create_ship(
            &player1,
            ShipType::PatrolBoat,
            Coordinate::new(5, 5),
            Orientation::Landscape,
        );

        let _ = game.create_ship(
            &player1,
            ShipType::Submarine,
            Coordinate::new(8, 7),
            Orientation::Landscape,
        );

        let _ = game.create_ship(
            &player2,
            ShipType::AircraftCarrier,
            Coordinate::new(2, 1),
            Orientation::Landscape,
        );

        let _ = game.create_ship(
            &player2,
            ShipType::Battleship,
            Coordinate::new(8, 1),
            Orientation::Portrait,
        );

        let _ = game.create_ship(
            &player2,
            ShipType::Destroyer,
            Coordinate::new(3, 8),
            Orientation::Portrait,
        );

        let _ = game.create_ship(
            &player2,
            ShipType::PatrolBoat,
            Coordinate::new(5, 5),
            Orientation::Landscape,
        );

        let _ = game.create_ship(
            &player2,
            ShipType::Submarine,
            Coordinate::new(8, 7),
            Orientation::Landscape,
        );
        game
    }
}
