#[derive(Copy, Clone, Debug)]
enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug)]
enum Piece {
    Pawn(Color),
}

#[derive(Copy, Clone, Debug)]
enum Square {
    Empty,
    Occupied(Piece),
}

fn main() {
    // let mut array: [i32; 3] = [0; 3];
    let board: [[Square; 4]; 4] = [
        [
            Square::Occupied(Piece::Pawn(Color::White)),
            Square::Occupied(Piece::Pawn(Color::White)),
            Square::Occupied(Piece::Pawn(Color::White)),
            Square::Occupied(Piece::Pawn(Color::White)),
        ],
        [Square::Empty, Square::Empty, Square::Empty, Square::Empty],
        [Square::Empty, Square::Empty, Square::Empty, Square::Empty],
        [
            Square::Occupied(Piece::Pawn(Color::Black)),
            Square::Occupied(Piece::Pawn(Color::Black)),
            Square::Occupied(Piece::Pawn(Color::Black)),
            Square::Occupied(Piece::Pawn(Color::Black)),
        ],
    ];

    println!("{:#?}", board);
}
