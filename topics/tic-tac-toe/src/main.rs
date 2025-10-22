mod game;

use game::{TicTacToe, Cell};
use std::io::{self, Write};

fn main() {
    let mut game = TicTacToe::new();
    let mut current_player = Cell::X;

    println!("=== Tic-Tac-Toe ===");
    println!("Joueur X commence !");
    println!("Entrez les coordonnées comme : ligne colonne (1-3)");
    println!("Exemple : 1 1 pour le coin supérieur gauche\n");

    loop {
        game.display();

        // Vérifier s'il y a un gagnant
        if let Some(winner) = game.check_winner() {
            println!("Le joueur {:?} a gagné !", winner);
            break;
        }

        // Vérifier si la grille est pleine
        if game.is_full() {
            println!("Match nul ! La grille est pleine.");
            break;
        }

        // Demander au joueur actuel de jouer
        println!("Joueur {:?}, entrez votre coup (ligne colonne) :", current_player);
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Parser l'entrée
        let coords: Vec<&str> = input.trim().split_whitespace().collect();
        if coords.len() != 2 {
            println!("❌ Entrée invalide ! Utilisez le format : ligne colonne (ex: 1 1)");
            continue;
        }

        let row = match coords[0].parse::<usize>() {
            Ok(r) if r >= 1 && r <= 3 => r - 1,
            Ok(_) => {
                println!("Ligne invalide ! Utilisez un nombre entre 1 et 3.");
                continue;
            }
            Err(_) => {
                println!("Ligne invalide !");
                continue;
            }
        };

        let col = match coords[1].parse::<usize>() {
            Ok(c) if c >= 1 && c <= 3 => c - 1,
            Ok(_) => {
                println!("Colonne invalide ! Utilisez un nombre entre 1 et 3.");
                continue;
            }
            Err(_) => {
                println!("Colonne invalide !");
                continue;
            }
        };

        // Tenter de placer le symbole
        if game.place(row, col, current_player) {
            // Changer de joueur
            current_player = match current_player {
                Cell::X => Cell::O,
                Cell::O => Cell::X,
                Cell::Empty => Cell::X,
            };
        } else {
            println!("Coup invalide ! La case est déjà occupée ou hors limites.");
        }
    }

    game.display();
    println!("\nFin de la partie");
}
