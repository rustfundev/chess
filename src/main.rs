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

    let moves: Vec<Notation> = read_notation()?
        .iter()
        .map(|x| get_move_from_notation(x))
        .flatten()
        .collect();

    println!("{:#?}", moves);

    Ok(())
}

fn get_notation_functions<'a>() -> Vec<&'a dyn Fn(&[NotationSymbol]) -> Notation> {
    let pawn_move = &Notation::pawn_move;
    let pawn_move_check = &Notation::pawn_move_check;
    let pawn_move_checkmate = &Notation::pawn_move_checkmate;
    let notation_fns: Vec<&dyn Fn(&[NotationSymbol]) -> Notation> =
        vec![pawn_move, pawn_move_check, pawn_move_checkmate];
    notation_fns
}

fn get_move_from_notation(symbols: &Vec<NotationSymbol>) -> Vec<Notation> {
    let notation = get_notation_functions()
        .iter()
        .map(|f| f(symbols))
        .filter(|n| !matches!(n, Notation::Unknown))
        .collect();

    notation
}

fn read_notation() -> Result<Vec<Vec<NotationSymbol>>, Box<dyn std::error::Error>> {
    let contents: Vec<String> = fs::read_to_string("notation.txt")?
        .lines()
        .map(String::from)
        .collect();

    let symbols: Vec<Vec<NotationSymbol>> = contents
        .iter()
        .map(|x| NotationSymbol::scan(x).unwrap())
        .collect();

    Ok(symbols)
}
