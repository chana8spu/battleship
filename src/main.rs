// Section numbers referenced in the comments and README of this program are from "Concepts of Programming Languages", Robert W. Sebesta

use rand::Rng;
use std::io::{self, Write};

// All types of cells are stored in an enumeration for ease of writability and readability. (6.4)
#[derive(Clone, Debug, PartialEq)]
enum Cell {
    Water,
    Ship,
    Hit,
    Miss,
}

// The Ship struct (a name for records (6.7)) is useful as it allows all of the necessary information to be stored in a single object (6.7). It also allows multiple ships to be created dynamically.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Ship {
    size: usize,
    positions: Vec<(usize, usize)>, // Vectors in Rust are heap-dynamic arrays (6.5.3), meaning they can grow and shrink throughout program execution
    sunk: bool, // Booleans have one of two values, true or false (6.2.2)
}

// Board is also a struct for similar reasons as Ship.
#[allow(dead_code)]
struct Board {
    grid: Vec<Vec<Cell>>,
    ships: Vec<Ship>,
    size: usize,
}

impl Board {
    fn new(size: usize, ship_sizes: Vec<usize>) -> Board {
        let mut grid = vec![vec![Cell::Water; size]; size];
        let mut ships = Vec::new();
        let mut rng = rand::thread_rng();

        for &ship_size in &ship_sizes {
            let mut placed = false;
            while !placed {
                let orientation = rng.gen_range(0..2); // 0 for horizontal, 1 for vertical
                let row = rng.gen_range(0..size);
                let col = rng.gen_range(0..size);

                // Prepare list of positions based on orientation
                let mut positions = Vec::new();
                if orientation == 0 {
                    // Horizontal ship
                    if col + ship_size <= size {
                        for i in 0..ship_size {
                            positions.push((row, col + i));
                        }
                    }
                } else {
                    // Vertical ship
                    if row + ship_size <= size {
                        for i in 0..ship_size {
                            positions.push((row + i, col));
                        }
                    }
                }

                // Retry placement if ship will extend out of bounds
                if positions.is_empty() {
                    continue;
                }
                // Check if existing ships block the placement
                if positions.iter().all(|&(r, c)| grid[r][c] == Cell::Water) {
                    // Place the ship on the grid
                    for &(r, c) in &positions {
                        grid[r][c] = Cell::Ship;
                    }
                    ships.push(Ship {size: ship_size, positions: positions.clone(), sunk: false});
                    placed = true;
                }
            }
        }        
        Board {grid, ships, size}
    }

    fn attack(&mut self, row: usize, col: usize) -> bool {
        match self.grid[row][col] {
            Cell::Ship => {
                println!("\nHit!");
                self.grid[row][col] = Cell::Hit;
                // Check if any ship has been sunk after the attack
                for ship in &mut self.ships {
                    if !ship.sunk && ship.positions.iter().all(|&(r, c)| self.grid[r][c] == Cell::Hit) {
                        ship.sunk = true;
                        println!("You've sunk a ship of size {}!", ship.size);
                    }
                }
                println!();
                true
            }
            Cell::Water => {
                println!("\nMiss!\n");
                self.grid[row][col] = Cell::Miss;
                false
            }
            _ => false,
        }
    }

    fn ships_remaining(&self) -> usize {
        self.ships.iter().filter(|ship| !ship.sunk).count()
    }

    fn print(&self, reveal_ships: bool) {
        // Print column headers
        println!("   0 1 2 3 4 5 6 7 8 9");

        for (i, row) in self.grid.iter().enumerate() {
            // Print row headers
            print!("{:2}", (b'A' + i as u8) as char); // Convert number to corresponding letter

            // Render grid, utilizing enumeration for ease of readability
            for cell in row.iter() {
                let symbol = match cell {
                    Cell::Water => " ~",
                    Cell::Ship => {
                        if reveal_ships {
                            " S"
                        } else {
                            " ~"
                        }
                    }
                    Cell::Hit => " X",
                    Cell::Miss => " O",
                };
                print!("{}", symbol);
            }
            println!();
        }
    }
}

// Ensure proper input
fn parse_input(input: &str) -> Option<(usize, usize)> {
    if input.len() < 2 {
        return None;
    }

    let input = input.trim().to_uppercase();
    let row_char = input.chars().next().unwrap();
    if row_char < 'A' || row_char > 'J' {
        return None;
    }
    let row = row_char as usize - 'A' as usize;

    let col = input[1..].parse::<usize>().ok()?;
    if col >= 10 {
        return None;
    }

    Some((row, col))
}

fn main() {
    let size = 10; // Changing board size is not supported
    let ship_sizes = vec![2, 3, 3, 4, 5]; // # of ships and their length can be changed, but having too many may make the board impossible to generate
    let max_turns = 40; // Number of turns can be changed
    let mut board = Board::new(size, ship_sizes.clone()); // Creates new board struct with the specified parameters
    let mut turns_left = max_turns;

    println!("\nWelcome to Battleship!\n");
    println!("The computer has placed down ships of these sizes: {:?}", ship_sizes);
    println!("Try to sink all of the ships in {} turns. Type a coordinate to attack it (ex. \"A4\", \"C7\").", max_turns);
    println!("~ = unknown | O = miss | X = hit\n");

    while turns_left > 0 && board.ships_remaining() > 0 {
        board.print(false);
        if turns_left > 5 {
            println!("You have {} turns left.", turns_left);
        } else if turns_left != 1 {
            println!("You have {} turns left. Make them count!", turns_left);
        } else {
            println!("You have 1 turn left. Make it count!");
        }

        let mut input = String::new();
        print!("Enter attack coordinates: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let (row, col) = match parse_input(&input) {
            Some((r, c)) => (r, c),
            None => {
                println!("Invalid input. Please enter coordinates (ex. \"A4\", \"C7\").\n");
                continue;
            }
        };

        board.attack(row, col);
        turns_left -= 1;
    }

    if board.ships_remaining() == 0 {
        println!("Congratulations! You've sunk all the ships in {} turns!\n", max_turns - turns_left);
    } else {
        println!("Game over! You've run out of turns.");
        println!("Unhit ship spaces are indicated with \"S\".\n");
    }

    // Reveal all ships at the end of the game
    board.print(true);
}