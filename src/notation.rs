const MAX_NOTATION_LENGTH: usize = 6;

#[derive(Debug)]
pub enum NotationSymbol {
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
    pub fn scan(input: &str) -> Result<Vec<NotationSymbol>, Box<dyn std::error::Error>> {
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
pub enum Notation {
    PawnMove(Option<NotationSymbol>, Option<NotationSymbol>),
    PawnMoveCheck(
        Option<NotationSymbol>,
        Option<NotationSymbol>,
        Option<NotationSymbol>,
    ),
    PawnMoveCheckmate(
        Option<NotationSymbol>,
        Option<NotationSymbol>,
        Option<NotationSymbol>,
    ),
    Unknown,
}

impl Notation {
    pub fn moves<'a>() -> Vec<&'a dyn Fn(&[NotationSymbol]) -> Notation> {
        vec![
            &Notation::pawn_move,
            &Notation::pawn_move_check,
            &Notation::pawn_move_checkmate,
        ]
    }
    pub fn pawn_move(symbols: &[NotationSymbol]) -> Notation {
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

    pub fn pawn_move_check(symbols: &[NotationSymbol]) -> Notation {
        let pawn_move = Notation::pawn_move(&symbols[0..2]);
        if matches!(pawn_move, Notation::Unknown) {
            return Notation::Unknown;
        }

        if symbols.len() == 3 && matches!(symbols[2], NotationSymbol::Check) {
            let mut rank: Option<NotationSymbol> = None;
            let mut file: Option<NotationSymbol> = None;
            if let Notation::PawnMove(v1, v2) = pawn_move {
                rank = v1;
                file = v2;
            }
            return Notation::PawnMoveCheck(rank, file, Some(NotationSymbol::Check));
        }

        return Notation::Unknown;
    }

    pub fn pawn_move_checkmate(symbols: &[NotationSymbol]) -> Notation {
        let pawn_move = Notation::pawn_move(&symbols[0..2]);
        if matches!(pawn_move, Notation::Unknown) {
            return Notation::Unknown;
        }

        if symbols.len() == 3 && matches!(symbols[2], NotationSymbol::Checkmate) {
            let mut rank: Option<NotationSymbol> = None;
            let mut file: Option<NotationSymbol> = None;
            if let Notation::PawnMove(v1, v2) = pawn_move {
                rank = v1;
                file = v2;
            }
            return Notation::PawnMoveCheckmate(rank, file, Some(NotationSymbol::Checkmate));
        }

        return Notation::Unknown;
    }
}
