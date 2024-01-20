#[derive(Copy, Clone, Debug)]
enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug)]
enum Piece {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

#[derive(Copy, Clone, Debug)]
enum Square {
    Empty,
    Occupied(Piece),
}

type Board = [[Square; 8]; 8];

#[derive(Debug)]
struct Game {
    board: Board,
}

impl Game {
    fn initialize(&mut self) -> () {
        self.board = [
            [
                Square::Occupied(Piece::Rook(Color::White)),
                Square::Occupied(Piece::Knight(Color::White)),
                Square::Occupied(Piece::Bishop(Color::White)),
                Square::Occupied(Piece::Queen(Color::White)),
                Square::Occupied(Piece::King(Color::White)),
                Square::Occupied(Piece::Bishop(Color::White)),
                Square::Occupied(Piece::Knight(Color::White)),
                Square::Occupied(Piece::Rook(Color::White)),
            ],
            [
                Square::Occupied(Piece::Pawn(Color::White)),
                Square::Occupied(Piece::Pawn(Color::White)),
                Square::Occupied(Piece::Pawn(Color::White)),
                Square::Occupied(Piece::Pawn(Color::White)),
                Square::Occupied(Piece::Pawn(Color::White)),
                Square::Occupied(Piece::Pawn(Color::White)),
                Square::Occupied(Piece::Pawn(Color::White)),
                Square::Occupied(Piece::Pawn(Color::White)),
            ],
            [
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
            ],
            [
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
            ],
            [
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
            ],
            [
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
            ],
            [
                Square::Occupied(Piece::Pawn(Color::Black)),
                Square::Occupied(Piece::Pawn(Color::Black)),
                Square::Occupied(Piece::Pawn(Color::Black)),
                Square::Occupied(Piece::Pawn(Color::Black)),
                Square::Occupied(Piece::Pawn(Color::Black)),
                Square::Occupied(Piece::Pawn(Color::Black)),
                Square::Occupied(Piece::Pawn(Color::Black)),
                Square::Occupied(Piece::Pawn(Color::Black)),
            ],
            [
                Square::Occupied(Piece::Rook(Color::Black)),
                Square::Occupied(Piece::Knight(Color::Black)),
                Square::Occupied(Piece::Bishop(Color::Black)),
                Square::Occupied(Piece::Queen(Color::Black)),
                Square::Occupied(Piece::King(Color::Black)),
                Square::Occupied(Piece::Bishop(Color::Black)),
                Square::Occupied(Piece::Knight(Color::Black)),
                Square::Occupied(Piece::Rook(Color::Black)),
            ],
        ];
    }
}

fn main() {
    let mut game = Game {
        board: [[Square::Empty; 8]; 8],
    };
    game.initialize();
    println!("{:#?}", game);
}
