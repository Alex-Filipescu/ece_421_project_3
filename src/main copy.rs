use std::io::{self, Write};

const WIDTH: usize = 7;
const HEIGHT: usize = 6;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Player {
    Red,
    Yellow,
}

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Occupied(Player),
}

struct Board {
    grid: [[Cell; WIDTH]; HEIGHT],
}

impl Board {
    fn new() -> Self {
        Board {
            grid: [[Cell::Empty; WIDTH]; HEIGHT],
        }
    }

    fn display(&self) {
        for row in self.grid.iter() {
            for cell in row.iter() {
                let symbol = match cell {
                    Cell::Empty => ".",
                    Cell::Occupied(Player::Red) => "R",
                    Cell::Occupied(Player::Yellow) => "Y",
                };
                print!("{} ", symbol);
            }
            println!();
        }
        println!("0 1 2 3 4 5 6");
    }

    fn drop_piece(&mut self, column: usize, player: Player) -> Result<(), &'static str> {
        if column >= WIDTH {
            return Err("Invalid column");
        }

        for row in (0..HEIGHT).rev() {
            if let Cell::Empty = self.grid[row][column] {
                self.grid[row][column] = Cell::Occupied(player);
                return Ok(());
            }
        }

        Err("Column is full")
    }

    fn check_winner(&self, player: Player) -> bool {
        // Check horizontal
        for row in 0..HEIGHT {
            for col in 0..WIDTH - 3 {
                if (0..4).all(|offset| self.grid[row][col + offset] == Cell::Occupied(player)) {
                    return true;
                }
            }
        }

        // Check vertical
        for col in 0..WIDTH {
            for row in 0..HEIGHT - 3 {
                if (0..4).all(|offset| self.grid[row + offset][col] == Cell::Occupied(player)) {
                    return true;
                }
            }
        }

        // Check diagonal (top-left to bottom-right)
        for row in 0..HEIGHT - 3 {
            for col in 0..WIDTH - 3 {
                if (0..4).all(|offset| self.grid[row + offset][col + offset] == Cell::Occupied(player)) {
                    return true;
                }
            }
        }

        // Check diagonal (bottom-left to top-right)
        for row in 3..HEIGHT {
            for col in 0..WIDTH - 3 {
                if (0..4).all(|offset| self.grid[row - offset][col + offset] == Cell::Occupied(player)) {
                    return true;
                }
            }
        }

        false
    }

    fn is_full(&self) -> bool {
        self.grid.iter().all(|row| row.iter().all(|&cell| cell != Cell::Empty))
    }
}

fn main() {
    let mut board = Board::new();
    let mut current_player = Player::Red;

    loop {
        println!("Current board:");
        board.display();

        println!("Player {:?}, choose a column (0-6):", current_player);
        io::stdout().flush().unwrap();

        let mut column_input = String::new();
        io::stdin().read_line(&mut column_input).expect("Failed to read input");
        let column: usize = match column_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number between 0 and 6.");
                continue;
            }
        };

        match board.drop_piece(column, current_player) {
            Ok(_) => {
                if board.check_winner(current_player) {
                    println!("Player {:?} wins!", current_player);
                    break;
                } else if board.is_full() {
                    println!("It's a draw!");
                    break;
                } else {
                    current_player = match current_player {
                        Player::Red => Player::Yellow,
                        Player::Yellow => Player::Red,
                    };
                }
            }
            Err(err) => println!("{}", err),
        }
    }
}
