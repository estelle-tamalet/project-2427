mod game;

use game::{TicTacToe, Cell};

fn main() {
    let mut game = TicTacToe::new();
    println!("=== Tic-Tac-Toe ===");
    game.display();

    // Exemple de jeu
    game.place(0, 0, Cell::X);
    game.place(1, 1, Cell::O);
    game.display();
}
