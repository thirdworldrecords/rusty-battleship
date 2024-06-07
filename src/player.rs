
use DisplayState::*;
use crate::game::{SIZE, Player, Coord, Orientation, NUM_SHIPS};
use Orientation::*;
use std::io::stdin;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq)]
pub enum DisplayState {
    Hit, Miss, Blank,
}

#[derive(Clone)]
pub struct PlayerView {
    pub state: [[DisplayState; SIZE]; SIZE]
}

use std::fmt;
impl fmt::Display for PlayerView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

        write!(f, "\n  |");

        for i in 1..=SIZE {
            write!(f, "{}|", i)?;
        }

        assert!(SIZE <= 26);
        for y in 0..SIZE {
            //Just let it crash or something if it's more than 26 lol
            write!(f, "\n{} |", alphabet.as_bytes()[y] as char)?;

            for x in 0..SIZE {
                let indicator = match self.state[x][y] {
                    Hit => "X",
                    Miss => "O",
                    Blank => " ",
                };

                write!(f, "{indicator}|")?;
            }
        }

        write!(f, " ")
    }
}

pub struct User {
    pub name: String,
    pub view: PlayerView,
}

/*
pub struct Computer {

}
*/

impl User {
    pub fn new(name: &str) -> User {
        User { 
            name: name.to_string(),
            view: PlayerView { state: [[Blank; SIZE]; SIZE] }
        }
    }
}

impl Player for User {

     fn place_ships(&self) -> [(usize, Coord, Orientation); NUM_SHIPS] {  
        let mut board = [[false; SIZE]; SIZE];
        let mut placement_view = PlayerView { state: [[Blank; SIZE]; SIZE] };

        let mut placements = [(0, Coord { x: 0, y: 0 },  Up); NUM_SHIPS];

        let mut ship_size = 1;

        while ship_size <= NUM_SHIPS {

            let ship_type_str = match ship_size {
                5 => String::from("Carrier"),
                4 => String::from("Battleship"),
                3 => String::from("Cruiser"),
                2 => String::from("Destroyer"),
                _ => format!("Size {ship_size}"),
            };

            println!("{}", placement_view);
            println!("Place your {ship_type_str} ship:\n");

            let mut input_str = String::new();

            stdin().read_line(&mut input_str).unwrap();
            let coord = Coord::from_str(&input_str);

            stdin().read_line(&mut input_str).unwrap();
            let orient: Result<Orientation, ()> = match input_str.as_str() {
                "Up" => Ok(Up),
                "Down" => Ok(Down),
                "Left" => Ok(Left),
                "Right" => Ok(Right),
                _ => Err(())
            };

            if coord.is_err() || orient.is_err() {
                println!("Usage: \n([x], [y])\n[Up, Down, Left, or Right]");
                continue;
            }

            let orient = orient.unwrap();
        

            let mut curr_coord = coord;
            let mut valid_flag = true;

            let mut new_board = board.clone();
            let mut new_view = placement_view.clone();

            for _ in 0..ship_size {
                if let Ok(new_coord) = curr_coord {
                    if new_coord.in_board() && !board[new_coord.x][new_coord.y] {
                        new_board[new_coord.x][new_coord.y] = true;
                        new_view[new_coord.x][new_coord.y] = Hit;
                        curr_coord = new_coord.shift(orient);
                        continue;
                    }
                }

                valid_flag = false;
                break;
            }

            if !valid_flag {
                println!("Invalid ship placement");
                continue;  
            }

            board = new_board;
            placement_view = new_view;

            placements[ship_size-1].0 = ship_size;
            placements[ship_size-1].1 = coord.unwrap();
            placements[ship_size-1].2 = orient;

            ship_size += 1;
        }

        placements
    }
    

    fn turn(&self) -> Coord {

        println!("{}", self.view);
        println!("\n{}'s turn\nEnter coordinates:\n", self.name);

        let mut fail_count: u32 = 0;

        loop {

            let mut coord_str = String::new();
            stdin().read_line(&mut coord_str).unwrap();
            let coord = Coord::from_str(&coord_str);

            if coord.is_ok() {
                return coord.unwrap();
            }

            fail_count += 1;

            match fail_count {
                1 => println!("Input error: format is (x, y), where x and y are integers from 1-{}", SIZE),
                2 => println!("Is it really that fucking hard to understand?"),
                3 => println!("Jesus fucking christ how hard is it to enter a fucking coordinate pair"),
                _ => panic!("\nError: user is too stupid to follow simple instructions\n"),
            }
        }
    }

    fn hit_feedback(&mut self, coord: Coord, hit: bool) {
        self.view.state[coord.x][coord.y] = if hit { Hit } else { Miss };
        println!("({}, {}) is a {}!", coord.x+1, coord.y+1, if hit { "hit" } else { "miss" });
    }

    fn count_hits(&self) -> usize {
        self.view.state.iter().flatten().filter(|&&state| state == Hit).count()
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

/*
impl Computer {
    
}

impl Player for Computer {
      fn place_ships(&self) -> [(usize, Coord, Orientation); NUM_SHIPS] {  
        let mut board = [[false; SIZE]; SIZE];
        let mut placements = [(0, Coord { x: 0, y: 0 },  Up); NUM_SHIPS];

        let mut ship_size = 1;

        while ship_size <= NUM_SHIPS {
            let mut rng = rand::thread_rng();

            let orient = match rng.gen_range(0..4) {
                0 => Up,
                1 => Down,
                2 => Left,
                3 => Right,
                _ => unreachable!(),
            };

            let start = Coord { x: rng.gen_range(0..SIZE), y: rng.gen_range(0..SIZE) };
            let mut curr_coord = Ok(start);
            let mut break_flag = false;

            let mut new_board = board.clone();

            for _ in 0..ship_size {
                if let Ok(new_coord) = curr_coord {
                    if new_coord.in_board() && !board[new_coord.x][new_coord.y] {
                        new_board[new_coord.x][new_coord.y] = true;
                        curr_coord = new_coord.shift(orient);
                        continue;
                    }
                }

                break_flag = true;
                break;
            }

            if break_flag { continue; }

            board = new_board;

            placements[ship_size-1].0 = ship_size;
            placements[ship_size-1].1 = start;
            placements[ship_size-1].2 = orient;

            ship_size += 1;
        }

        placements
    }
    
}
*/
