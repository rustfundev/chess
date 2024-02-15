mod game;
mod notation;

use game::{Game, Square};
use notation::{Notation, NotationSymbol};

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut game = Game {
        board: [[Square::Empty; 8]; 8],
    };
    game.initialize();

    let read_notation_file = || -> Vec<String> {
        fs::read_to_string("notation.txt")
            .expect("error reading notation file")
            .lines()
            .map(String::from)
            .collect()
    };

    let parse_notation = || -> Vec<Vec<NotationSymbol>> {
        read_notation_file()
            .iter()
            .map(|x| NotationSymbol::scan(x).unwrap())
            .collect()
    };

    let validate_move = |m: &Vec<NotationSymbol>| -> Vec<Notation> {
        Notation::moves()
            .iter()
            .map(|f| f(m))
            .filter(|n| !matches!(n, Notation::Unknown))
            .collect()
    };

    let moves: Vec<Notation> = parse_notation()
        .iter()
        .map(|m| validate_move(m))
        .flatten()
        .collect();

    println!("{:#?}", moves);

    Ok(())
}
