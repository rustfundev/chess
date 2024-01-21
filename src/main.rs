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

struct Move(String);

impl Move {
    fn from(&self) -> (usize, usize) {
        (1, 0)
    }
    fn to(&self) -> (usize, usize) {
        (2, 0)
    }
}

impl Game {
    fn play(&mut self, mov: Move) -> () {
        let rank_from = mov.from().0;
        let file_from = mov.from().1;
        let rank_to = mov.to().0;
        let file_to = mov.to().1;
        println!("{:#?}", self.board[rank_to][file_to]);
        self.board[rank_to][file_to] = self.board[rank_from][file_from];
        self.board[rank_from][file_from] = Square::Empty;
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
    game.play(Move(String::from("h5")));
    println!("{:#?}", game.board);
}
