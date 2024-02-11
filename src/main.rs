use std::collections::btree_set::Union;

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

const MAX_NOTATION_LENGTH: usize = 6;

#[derive(Debug)]
enum NotationSymbol {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Capture,
    Check,
    Checkmate,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Minus,
}

impl NotationSymbol {
    fn scan(input: &str) -> Result<Vec<NotationSymbol>, Box<dyn std::error::Error>> {
        let mut symbols: Vec<NotationSymbol> = Vec::new();
        let mut invalid_symbol: bool = false;
        let mut invalid_length: bool = false;
        String::from(input).chars().for_each(|c| {
            match c {
                'a' => symbols.push(NotationSymbol::A),
                'b' => symbols.push(NotationSymbol::B),
                'c' => symbols.push(NotationSymbol::C),
                'd' => symbols.push(NotationSymbol::D),
                'e' => symbols.push(NotationSymbol::E),
                'f' => symbols.push(NotationSymbol::F),
                'g' => symbols.push(NotationSymbol::G),
                'h' => symbols.push(NotationSymbol::H),
                'B' => symbols.push(NotationSymbol::Bishop),
                'N' => symbols.push(NotationSymbol::Knight),
                'R' => symbols.push(NotationSymbol::Rook),
                'K' => symbols.push(NotationSymbol::King),
                'Q' => symbols.push(NotationSymbol::Queen),
                'x' => symbols.push(NotationSymbol::Capture),
                '+' => symbols.push(NotationSymbol::Check),
                '#' => symbols.push(NotationSymbol::Checkmate),
                '-' => symbols.push(NotationSymbol::Minus),
                '0' => symbols.push(NotationSymbol::Zero),
                '1' => symbols.push(NotationSymbol::One),
                '2' => symbols.push(NotationSymbol::Two),
                '3' => symbols.push(NotationSymbol::Three),
                '4' => symbols.push(NotationSymbol::Four),
                '5' => symbols.push(NotationSymbol::Five),
                '6' => symbols.push(NotationSymbol::Six),
                '7' => symbols.push(NotationSymbol::Seven),
                '8' => symbols.push(NotationSymbol::Eight),
                _ => invalid_symbol = true,
            }
            if symbols.len() > MAX_NOTATION_LENGTH {
                invalid_length = true;
            }

            if invalid_symbol || invalid_length {
                return ();
            }
        });
        if invalid_symbol {
            return Err("invalid notation symbol".into());
        }
        if invalid_length {
            return Err("invalid notation length".into());
        }
        Ok(symbols)
    }
}

// N -> PAWN_MOVE |
//      PAWN_MOVE_CAPTURE |
//      PAWN_MOVE_CHECK |
//      PAWN_MOVE_CHECKMATE |
//      PAWN_MOVE_PROMOTION |
//      OTHER_MOVE |
//      OTHER_MOVE_CAPTURE |
//      OTHER_MOVE_CHECK |
//      OTHER_MOVE_CHECKMATE |
//      CASTLING_KING_SIDE |
//      CASTLING_QUEEN_SIDE

#[derive(Debug)]
enum Notation {
    PawnMove(Option<NotationSymbol>, Option<NotationSymbol>),
    Unknown,
}

impl Notation {
    fn pawn_move(symbols: &Vec<NotationSymbol>) -> Notation {
        if symbols.len() != 2 {
            return Notation::Unknown;
        }

        let rank: Option<NotationSymbol>;
        let file: Option<NotationSymbol>;

        match symbols[0] {
            NotationSymbol::A => rank = Some(NotationSymbol::A),
            NotationSymbol::B => rank = Some(NotationSymbol::B),
            NotationSymbol::C => rank = Some(NotationSymbol::C),
            NotationSymbol::D => rank = Some(NotationSymbol::D),
            NotationSymbol::E => rank = Some(NotationSymbol::E),
            NotationSymbol::F => rank = Some(NotationSymbol::F),
            NotationSymbol::G => rank = Some(NotationSymbol::G),
            NotationSymbol::H => rank = Some(NotationSymbol::H),
            _ => rank = None,
        };

        match symbols[1] {
            NotationSymbol::One => file = Some(NotationSymbol::One),
            NotationSymbol::Two => file = Some(NotationSymbol::Two),
            NotationSymbol::Three => file = Some(NotationSymbol::Three),
            NotationSymbol::Four => file = Some(NotationSymbol::Four),
            NotationSymbol::Five => file = Some(NotationSymbol::Five),
            NotationSymbol::Six => file = Some(NotationSymbol::Six),
            NotationSymbol::Seven => file = Some(NotationSymbol::Seven),
            NotationSymbol::Eight => file = Some(NotationSymbol::Eight),
            _ => file = None,
        };

        if rank.is_none() || file.is_none() {
            return Notation::Unknown;
        }

        Notation::PawnMove(rank, file)
    }
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut game = Game {
        board: [[Square::Empty; 8]; 8],
    };
    game.initialize();
    let tokens = NotationSymbol::scan("a1")?;
    let pawn_move = &Notation::pawn_move;
    let notations: Vec<&dyn Fn(&Vec<NotationSymbol>) -> Notation> = vec![pawn_move];

    let mut notation = Notation::Unknown;
    notations.iter().for_each(|f| {
        notation = f(&tokens);
    });

    println!("{:#?}", notation);
    Ok(())
}
