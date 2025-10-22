const SIZE: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    X,
    O,
}

pub struct TicTacToe {
    board: [[Cell; SIZE]; SIZE],
}

impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {
            board: [[Cell::Empty; SIZE]; SIZE],
        }
    }

    pub fn display(&self) {
        println!();
        for (i, row) in self.board.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let symbol = match cell {
                    Cell::Empty => ".",
                    Cell::X => "X",
                    Cell::O => "O",
                };
                print!("{}", symbol);
                if j < SIZE - 1 {
                    print!(" | ");
                }
            }
            println!();
            if i < SIZE - 1 {
                println!("---------");
            }
        }
        println!();
    }

    pub fn place(&mut self, row: usize, col: usize, player: Cell) -> bool {
        if row < SIZE && col < SIZE && self.board[row][col] == Cell::Empty {
            self.board[row][col] = player;
            true
        } else {
            false
        }
    }

    pub fn is_full(&self) -> bool {
        self.board.iter().all(|row| row.iter().all(|&cell| cell != Cell::Empty))
    }

    pub fn check_winner(&self) -> Option<Cell> {
        // Vérifier les lignes et colonnes
        for i in 0..SIZE {
            if self.board[i][0] != Cell::Empty
                && self.board[i][0] == self.board[i][1]
                && self.board[i][1] == self.board[i][2]
            {
                return Some(self.board[i][0]);
            }
            if self.board[0][i] != Cell::Empty
                && self.board[0][i] == self.board[1][i]
                && self.board[1][i] == self.board[2][i]
            {
                return Some(self.board[0][i]);
            }
        }

        // Vérifier les diagonales
        if self.board[0][0] != Cell::Empty
            && self.board[0][0] == self.board[1][1]
            && self.board[1][1] == self.board[2][2]
        {
            return Some(self.board[0][0]);
        }
        if self.board[0][2] != Cell::Empty
            && self.board[0][2] == self.board[1][1]
            && self.board[1][1] == self.board[2][0]
        {
            return Some(self.board[0][2]);
        }

        None
    }
}
