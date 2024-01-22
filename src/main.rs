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

#[derive(Debug)]
enum Notation {
    Pawn(char, u8, Option<char>),                // e4, e4+, e4#
    Regular(char, char, u8, Option<char>),       // Nf3, Nf3+, Nf3#
    Capture(char, char, char, u8, Option<char>), // Bxc6, Bxc6+, Bxc6#
    ShortCastling(u8, char, u8),                 // 0-0
    LongCastling(u8, char, u8, char, u8),        // 0-0-0
}

type Board = [[Square; 8]; 8];

#[derive(Debug)]
struct Game {
    board: Board,
}

impl Game {
    fn play(&mut self, next_move: &str) -> () {
        for c in next_move.chars() {
            println!("{}", c as u8);
        }

        //       let rank_from = mov.from().0;
        //       let file_from = mov.from().1;
        //       let rank_to = mov.to().0;
        //       let file_to = mov.to().1;
        //       self.board[rank_to][file_to] = self.board[rank_from][file_from];
        //       self.board[rank_from][file_from] = Square::Empty;
    }
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
    game.play("h5");
    println!("{:#?}", Notation::Pawn('h', 5, None));
    println!("{:#?}", Notation::Capture('N', 'x', 'f', 4, Some('+')));
    println!("{:#?}", Notation::ShortCastling(0, '-', 0));
}
