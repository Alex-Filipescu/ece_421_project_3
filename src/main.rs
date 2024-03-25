use std::io;

const ROWS: usize = 6;
const COLS: usize = 7;
const WORDS: [&str; 2] = ["TOOT", "OTTO"];

struct Game {
    board: [[Option<char>; COLS]; ROWS],
    current_player: char,
}

impl Game {
    fn new() -> Game {
        Game {
            board: [[None; COLS]; ROWS],
            current_player: '1',
        }
    }

    fn display_board(&self) {
        for row in &self.board {
            let row_str: String = format!(" {}", row.iter()
                .map(|&cell| match cell {
                    Some(c) => c.to_string(),
                    None => ' '.to_string(),
                })
                .collect::<Vec<_>>()
                .join(" "));
            println!("{}", row_str);
        }
        println!("---------------");
        println!(" 0 1 2 3 4 5 6");
        println!("---------------");
    }

    fn make_move(&mut self, column: usize, disc: char) -> Result<(), &'static str> {
        if column >= COLS {
            return Err("Invalid column");
        }
        for i in (0..ROWS).rev() {
            if self.board[i][column].is_none() {
                self.board[i][column] = Some(disc);
                return Ok(());
            }
        }
        Err("Column full")
    }

    fn check_win(&self, row: usize, col: usize) -> Option<String> {
        // Check for wins in rows
        for r in 0..ROWS {
            let row_str: String = self.board[r].iter().map(|&cell| cell.unwrap_or(' ')).collect();
            if row_str.contains("TOOT") || row_str.contains("OTTO") {
                return Some(row_str);
            }
        }
    
        // Check for wins in columns
        for c in 0..COLS {
            let col_str: String = (0..ROWS).map(|r| self.board[r][c].unwrap_or(' ')).collect();
            if col_str.contains("TOOT") || col_str.contains("OTTO") {
                return Some(col_str);
            }
        }
    
        // Check for wins in diagonals
        for r in 0..ROWS {
            for c in 0..COLS {
                if let Some(cell) = self.board[r][c] {
                    let mut diag_str1 = String::new();
                    let mut diag_str2 = String::new();
                    
                    // Diagonal from top-left to bottom-right
                    for i in 0..4 {
                        if r + i < ROWS && c + i < COLS {
                            diag_str1.push(self.board[r + i][c + i].unwrap_or(' '));
                        }
                    }
    
                    // Diagonal from top-right to bottom-left
                    for i in 0..4 {
                        if r + i < ROWS && c as isize - i as isize >= 0 {
                            diag_str2.push(self.board[r + i][c - i].unwrap_or(' '));
                        }
                    }
    
                    if diag_str1.contains("TOOT") || diag_str1.contains("OTTO") {
                        return Some(diag_str1);
                    }
                    if diag_str2.contains("TOOT") || diag_str2.contains("OTTO") {
                        return Some(diag_str2);
                    }
                }
            }
        }
    
        None
    }

    fn switch_player(&mut self) {
        self.current_player = if self.current_player == '1' { '2' } else { '1' };
    }

    fn play(&mut self) {
        loop {
            println!("Player 1: OTTO\nPlayer 2: TOOT");

            self.display_board();
            println!("Player {}'s turn:", self.current_player);
            println!("Enter column number (0-6):");

            let mut column = String::new();
            io::stdin().read_line(&mut column).expect("Failed to read line");
            let column: usize = match column.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number!");
                    continue;
                }
            };

            println!("Enter disk type (T/O):");
            let mut disk_type = String::new();
            io::stdin().read_line(&mut disk_type).expect("Failed to read line");
            let disk_type = match disk_type.trim().to_uppercase().as_str() {
                "T" => 'T',
                "O" => 'O',
                _ => {
                    println!("Invalid disk type");
                    continue;
                }
            };

            match self.make_move(column, disk_type) {
                Ok(()) => {
                    if let Some(winning_sequence) = self.check_win(ROWS - 1, column) {
                        self.display_board();
                        if winning_sequence.trim() == "TOOT" {
                            println!("Player 2 wins!");
                        } else {
                            println!("Player 1 wins!");
                        }
                        
                        break;
                    }
                }
                Err(msg) => println!("{}", msg),
            }
        }
    }
}

fn main() {
    let mut game = Game::new();
    game.play();
}
