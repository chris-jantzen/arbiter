use std::str::Split;

mod engine;

pub enum Color {
    White,
    Black,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Color::White => String::from("w"),
            Color::Black => String::from("b"),
        }
    }
}

pub enum Piece {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

#[rustfmt::skip]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

pub struct Game {
    board: [[Option<Piece>; 8]; 8],
    active_color: Color,
    white_can_castle_kingside: bool,
    white_can_castle_queenside: bool,
    black_can_castle_kingside: bool,
    black_can_castle_queenside: bool,
    en_passant_target: Option<Square>,
    half_move_clock: u8,
    full_move_number: u16,
}

impl Piece {
    fn from_char(piece: &char) -> Option<Piece> {
        match piece {
            'r' => Some(Piece::Rook(Color::Black)),
            'n' => Some(Piece::Knight(Color::Black)),
            'b' => Some(Piece::Bishop(Color::Black)),
            'q' => Some(Piece::Queen(Color::Black)),
            'k' => Some(Piece::King(Color::Black)),
            'p' => Some(Piece::Pawn(Color::Black)),
            'P' => Some(Piece::Pawn(Color::White)),
            'R' => Some(Piece::Rook(Color::White)),
            'N' => Some(Piece::Knight(Color::White)),
            'B' => Some(Piece::Bishop(Color::White)),
            'Q' => Some(Piece::Queen(Color::White)),
            'K' => Some(Piece::King(Color::White)),
            '1'..='8' => None,
            _ => panic!("Invalid character entered in piece placement data."),
        }
    }
}

impl ToString for Square {
    fn to_string(&self) -> String {
        String::from(match self {
            Square::A1 => "a1",
            Square::A2 => "a2",
            Square::A3 => "a3",
            Square::A4 => "a4",
            Square::A5 => "a5",
            Square::A6 => "a6",
            Square::A7 => "a7",
            Square::A8 => "a8",
            Square::B1 => "b1",
            Square::B2 => "b2",
            Square::B3 => "b3",
            Square::B4 => "b4",
            Square::B5 => "b5",
            Square::B6 => "b6",
            Square::B7 => "b7",
            Square::B8 => "b8",
            Square::C1 => "c1",
            Square::C2 => "c2",
            Square::C3 => "c3",
            Square::C4 => "c4",
            Square::C5 => "c5",
            Square::C6 => "c6",
            Square::C7 => "c7",
            Square::C8 => "c8",
            Square::D1 => "d1",
            Square::D2 => "d2",
            Square::D3 => "d3",
            Square::D4 => "d4",
            Square::D5 => "d5",
            Square::D6 => "d6",
            Square::D7 => "d7",
            Square::D8 => "d8",
            Square::E1 => "e1",
            Square::E2 => "e2",
            Square::E3 => "e3",
            Square::E4 => "e4",
            Square::E5 => "e5",
            Square::E6 => "e6",
            Square::E7 => "e7",
            Square::E8 => "e8",
            Square::F1 => "f1",
            Square::F2 => "f2",
            Square::F3 => "f3",
            Square::F4 => "f4",
            Square::F5 => "f5",
            Square::F6 => "f6",
            Square::F7 => "f7",
            Square::F8 => "f8",
            Square::G1 => "g1",
            Square::G2 => "g2",
            Square::G3 => "g3",
            Square::G4 => "g4",
            Square::G5 => "g5",
            Square::G6 => "g6",
            Square::G7 => "g7",
            Square::G8 => "g8",
            Square::H1 => "h1",
            Square::H2 => "h2",
            Square::H3 => "h3",
            Square::H4 => "h4",
            Square::H5 => "h5",
            Square::H6 => "h6",
            Square::H7 => "h7",
            Square::H8 => "h8",
        })
    }
}

trait FromStr {
    fn from_str(str: &str) -> Option<Self>
    where
        Self: Sized;
}

impl FromStr for Square {
    fn from_str(str: &str) -> Option<Square> {
        match str {
            "a1" => Some(Square::A1),
            "a2" => Some(Square::A2),
            "a3" => Some(Square::A3),
            "a4" => Some(Square::A4),
            "a5" => Some(Square::A5),
            "a6" => Some(Square::A6),
            "a7" => Some(Square::A7),
            "a8" => Some(Square::A8),
            "b1" => Some(Square::B1),
            "b2" => Some(Square::B2),
            "b3" => Some(Square::B3),
            "b4" => Some(Square::B4),
            "b5" => Some(Square::B5),
            "b6" => Some(Square::B6),
            "b7" => Some(Square::B7),
            "b8" => Some(Square::B8),
            "c1" => Some(Square::C1),
            "c2" => Some(Square::C2),
            "c3" => Some(Square::C3),
            "c4" => Some(Square::C4),
            "c5" => Some(Square::C5),
            "c6" => Some(Square::C6),
            "c7" => Some(Square::C7),
            "c8" => Some(Square::C8),
            "d1" => Some(Square::D1),
            "d2" => Some(Square::D2),
            "d3" => Some(Square::D3),
            "d4" => Some(Square::D4),
            "d5" => Some(Square::D5),
            "d6" => Some(Square::D6),
            "d7" => Some(Square::D7),
            "d8" => Some(Square::D8),
            "e1" => Some(Square::E1),
            "e2" => Some(Square::E2),
            "e3" => Some(Square::E3),
            "e4" => Some(Square::E4),
            "e5" => Some(Square::E5),
            "e6" => Some(Square::E6),
            "e7" => Some(Square::E7),
            "e8" => Some(Square::E8),
            "f1" => Some(Square::F1),
            "f2" => Some(Square::F2),
            "f3" => Some(Square::F3),
            "f4" => Some(Square::F4),
            "f5" => Some(Square::F5),
            "f6" => Some(Square::F6),
            "f7" => Some(Square::F7),
            "f8" => Some(Square::F8),
            "g1" => Some(Square::G1),
            "g2" => Some(Square::G2),
            "g3" => Some(Square::G3),
            "g4" => Some(Square::G4),
            "g5" => Some(Square::G5),
            "g6" => Some(Square::G6),
            "g7" => Some(Square::G7),
            "g8" => Some(Square::G8),
            "h1" => Some(Square::H1),
            "h2" => Some(Square::H2),
            "h3" => Some(Square::H3),
            "h4" => Some(Square::H4),
            "h5" => Some(Square::H5),
            "h6" => Some(Square::H6),
            "h7" => Some(Square::H7),
            "h8" => Some(Square::H8),
            "-" => None,
            _ => panic!("En Passant Target Required in FEN string"),
        }
    }
}

impl ToString for Piece {
    fn to_string(&self) -> String {
        String::from(match self {
            Piece::Pawn(Color::Black) => "p",
            Piece::Rook(Color::Black) => "r",
            Piece::Knight(Color::Black) => "n",
            Piece::Bishop(Color::Black) => "b",
            Piece::Queen(Color::Black) => "q",
            Piece::King(Color::Black) => "k",
            Piece::Pawn(Color::White) => "P",
            Piece::Rook(Color::White) => "R",
            Piece::Knight(Color::White) => "N",
            Piece::Bishop(Color::White) => "B",
            Piece::Queen(Color::White) => "Q",
            Piece::King(Color::White) => "K",
        })
    }
}

impl Game {
    pub fn from_fen(fen: &str) -> Game {
        let fen = String::from(fen);
        let mut fen_fields = fen.split_whitespace();
        let rows = fen_fields
            .next()
            .expect("Incorrectly formatted string")
            .split("/");
        let board = Game::assemble_board(rows);
        let active_color = match fen_fields.next() {
            Some("w") => Color::White,
            Some("b") => Color::Black,
            None => panic!("Must have active color in FEN"),
            _ => panic!("Must have active color in FEN - valid values are either 'w' or 'b'"),
        };

        let castleable = fen_fields
            .next()
            .expect("Castling Availability Required")
            .chars();

        let mut white_can_castle_kingside = false;
        let mut white_can_castle_queenside = false;
        let mut black_can_castle_kingside = false;
        let mut black_can_castle_queenside = false;
        for castle in castleable {
            if castle == '-' {
                break;
            } else if castle == 'k' {
                black_can_castle_kingside = true;
            } else if castle == 'q' {
                black_can_castle_queenside = true;
            } else if castle == 'K' {
                white_can_castle_kingside = true;
            } else if castle == 'Q' {
                white_can_castle_queenside = true;
            }
        }

        let target_square = fen_fields.next();
        let en_passant_target =
            Square::from_str(target_square.expect("En Passant Target Required in FEN string"));

        let half_move_clock = fen_fields
            .next()
            .expect("Must provide half move clock value in FEN")
            .parse::<u8>()
            .expect("Could not parse half move clock value");

        let full_move_number = fen_fields
            .next()
            .expect("Must provide full move number value in FEN")
            .parse::<u16>()
            .expect("Could not parse full move number value");

        Game {
            board,
            active_color,
            white_can_castle_kingside,
            white_can_castle_queenside,
            black_can_castle_kingside,
            black_can_castle_queenside,
            en_passant_target,
            half_move_clock,
            full_move_number,
        }
    }

    pub fn to_fen(&self) -> String {
        let pieces = Game::board_to_piece_string(&self);
        let active_color = Color::to_string(&self.active_color);

        let castleable = self.castleable_to_string();

        let en_passant_square = match &self.en_passant_target {
            Some(square) => square.to_string(),
            None => String::from("-"),
        };
        let half_move = &self.half_move_clock.to_string();
        let full_move = &self.full_move_number.to_string();

        format!("{pieces} {active_color} {castleable} {en_passant_square} {half_move} {full_move}")
    }

    pub fn make_move(&mut self, from: Square, to: Square) -> bool {
        true
    }

    fn assemble_board(rows: Split<&str>) -> [[Option<Piece>; 8]; 8] {
        let mut board: [[Option<Piece>; 8]; 8] = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];

        let mut skipped_spaces: usize = 0;
        for (row_index, row) in rows.enumerate() {
            for (piece_index, piece) in row.chars().enumerate() {
                let board_position = Piece::from_char(&piece);
                if board_position.is_none() {
                    skipped_spaces += (piece.to_string()).parse::<usize>().unwrap() - 1;
                }
                board[row_index][piece_index + skipped_spaces] = board_position;
            }
            skipped_spaces = 0;
        }

        board
    }

    fn board_to_piece_string(&self) -> String {
        // Count of how many spaces in the row since last piece
        let mut num = 0;
        let mut pieces = String::new();

        for row in &self.board {
            for piece in row {
                let p = match piece {
                    Some(p) => p.to_string(),
                    None => String::from("empty"),
                };
                if p != "empty" {
                    if num > 0 {
                        pieces += &num.to_string();
                        num = 0;
                    }
                    pieces += &p;
                } else {
                    num += 1;
                }
            }
            if num != 0 {
                pieces += &num.to_string();
            }
            pieces += "/";
            num = 0;
        }
        // Trim the last /
        pieces[..pieces.len() - 1].to_string()
    }

    fn castleable_to_string(&self) -> String {
        let mut castleable = String::new();
        if self.white_can_castle_kingside {
            castleable += "K";
        }
        if self.white_can_castle_queenside {
            castleable += "Q";
        }
        if self.black_can_castle_kingside {
            castleable += "k";
        }
        if self.black_can_castle_queenside {
            castleable += "q";
        }
        if castleable.len() == 0 {
            castleable += "-";
        }
        castleable
    }
}

impl Default for Game {
    fn default() -> Game {
        Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_fen_base() {
        let game = Game::default();

        assert_eq!(
            game.to_fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
    }

    #[test]
    fn to_fen_with_moves_no_takes() {
        let fen = "rnbqkb1r/pp3ppp/2pp1n2/1B2p3/4P3/2N2N2/PPPP1PPP/R1BQ1RK1 b kq - 1 5";
        let game = Game::from_fen(fen);
        assert_eq!(game.to_fen(), fen);
    }

    #[test]
    fn to_fen_with_moves_and_takes() {
        let fen = "rnbk1b1r/ppNq1ppp/8/1B6/3Pp3/5N2/PPP2PPP/R1BQK2R b KQ d3 0 8";
        let game = Game::from_fen(fen);
        assert_eq!(game.to_fen(), fen);
    }

    #[test]
    fn from_fen_base_success() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let res = Game::from_fen(fen);

        assert_eq!(res.active_color.to_string(), String::from("w"));

        assert_eq!(res.half_move_clock, 0);
        assert_eq!(res.full_move_number, 1);

        assert_eq!(res.white_can_castle_kingside, true);
        assert_eq!(res.white_can_castle_queenside, true);
        assert_eq!(res.black_can_castle_kingside, true);
        assert_eq!(res.black_can_castle_queenside, true);

        assert!(matches!(res.en_passant_target, None));

        // Black Pieces
        assert!(matches!(res.board[0][0], Some(Piece::Rook(Color::Black))));
        assert!(matches!(res.board[0][1], Some(Piece::Knight(Color::Black))));
        assert!(matches!(res.board[0][2], Some(Piece::Bishop(Color::Black))));
        assert!(matches!(res.board[0][3], Some(Piece::Queen(Color::Black))));
        assert!(matches!(res.board[0][4], Some(Piece::King(Color::Black))));
        assert!(matches!(res.board[0][5], Some(Piece::Bishop(Color::Black))));
        assert!(matches!(res.board[0][6], Some(Piece::Knight(Color::Black))));
        assert!(matches!(res.board[0][7], Some(Piece::Rook(Color::Black))));

        // White Pieces
        assert!(matches!(res.board[7][0], Some(Piece::Rook(Color::White))));
        assert!(matches!(res.board[7][1], Some(Piece::Knight(Color::White))));
        assert!(matches!(res.board[7][2], Some(Piece::Bishop(Color::White))));
        assert!(matches!(res.board[7][3], Some(Piece::Queen(Color::White))));
        assert!(matches!(res.board[7][4], Some(Piece::King(Color::White))));
        assert!(matches!(res.board[7][5], Some(Piece::Bishop(Color::White))));
        assert!(matches!(res.board[7][6], Some(Piece::Knight(Color::White))));
        assert!(matches!(res.board[7][7], Some(Piece::Rook(Color::White))));

        for i in 0..8 {
            assert!(matches!(res.board[1][i], Some(Piece::Pawn(Color::Black))));
            assert!(matches!(res.board[6][i], Some(Piece::Pawn(Color::White))));
        }

        for i in 2..6 {
            for j in 0..8 {
                assert!(matches!(res.board[i][j], None));
            }
        }
    }

    #[test]
    fn from_fen_with_moves_no_takes() {
        let fen = "rnbqkb1r/pp3ppp/2pp1n2/1B2p3/4P3/2N2N2/PPPP1PPP/R1BQ1RK1 b kq - 1 5";

        let res = Game::from_fen(fen);

        assert_eq!(res.active_color.to_string(), String::from("b"));

        assert_eq!(res.half_move_clock, 1);
        assert_eq!(res.full_move_number, 5);

        assert_eq!(res.white_can_castle_kingside, false);
        assert_eq!(res.white_can_castle_queenside, false);
        assert_eq!(res.black_can_castle_kingside, true);
        assert_eq!(res.black_can_castle_queenside, true);

        assert!(matches!(res.en_passant_target, None));

        // Rooks
        assert!(matches!(res.board[0][0], Some(Piece::Rook(Color::Black))));
        assert!(matches!(res.board[0][7], Some(Piece::Rook(Color::Black))));
        assert!(matches!(res.board[7][5], Some(Piece::Rook(Color::White))));
        assert!(matches!(res.board[7][0], Some(Piece::Rook(Color::White))));

        // Knights
        assert!(matches!(res.board[0][1], Some(Piece::Knight(Color::Black))));
        assert!(matches!(res.board[2][5], Some(Piece::Knight(Color::Black))));
        assert!(matches!(res.board[5][2], Some(Piece::Knight(Color::White))));
        assert!(matches!(res.board[5][5], Some(Piece::Knight(Color::White))));

        // Bishops
        assert!(matches!(res.board[0][2], Some(Piece::Bishop(Color::Black))));
        assert!(matches!(res.board[0][5], Some(Piece::Bishop(Color::Black))));
        assert!(matches!(res.board[7][2], Some(Piece::Bishop(Color::White))));
        assert!(matches!(res.board[3][1], Some(Piece::Bishop(Color::White))));

        // Queens
        assert!(matches!(res.board[0][3], Some(Piece::Queen(Color::Black))));
        assert!(matches!(res.board[7][3], Some(Piece::Queen(Color::White))));

        // Kings
        assert!(matches!(res.board[0][4], Some(Piece::King(Color::Black))));
        assert!(matches!(res.board[7][6], Some(Piece::King(Color::White))));

        // Pawns
        assert!(matches!(res.board[1][0], Some(Piece::Pawn(Color::Black))));
        assert!(matches!(res.board[1][1], Some(Piece::Pawn(Color::Black))));
        assert!(matches!(res.board[2][2], Some(Piece::Pawn(Color::Black))));
        assert!(matches!(res.board[2][3], Some(Piece::Pawn(Color::Black))));
        assert!(matches!(res.board[3][4], Some(Piece::Pawn(Color::Black))));
        assert!(matches!(res.board[1][5], Some(Piece::Pawn(Color::Black))));
        assert!(matches!(res.board[1][6], Some(Piece::Pawn(Color::Black))));
        assert!(matches!(res.board[1][7], Some(Piece::Pawn(Color::Black))));
        assert!(matches!(res.board[6][0], Some(Piece::Pawn(Color::White))));
        assert!(matches!(res.board[6][1], Some(Piece::Pawn(Color::White))));
        assert!(matches!(res.board[6][2], Some(Piece::Pawn(Color::White))));
        assert!(matches!(res.board[6][3], Some(Piece::Pawn(Color::White))));
        assert!(matches!(res.board[4][4], Some(Piece::Pawn(Color::White))));
        assert!(matches!(res.board[6][5], Some(Piece::Pawn(Color::White))));
        assert!(matches!(res.board[6][6], Some(Piece::Pawn(Color::White))));
        assert!(matches!(res.board[6][7], Some(Piece::Pawn(Color::White))));

        // None
        assert!(matches!(res.board[0][6], None));
        assert!(matches!(res.board[1][2], None));
        assert!(matches!(res.board[1][3], None));
        assert!(matches!(res.board[1][4], None));
        vec![0, 1, 4, 6, 7]
            .iter()
            .for_each(|&s| assert!(matches!(res.board[2][s], None)));
        vec![0, 2, 3, 5, 6, 7]
            .iter()
            .for_each(|&s| assert!(matches!(res.board[3][s], None)));
        vec![0, 1, 2, 3, 5, 6, 7]
            .iter()
            .for_each(|&s| assert!(matches!(res.board[4][s], None)));
        vec![0, 1, 3, 4, 6, 7]
            .iter()
            .for_each(|&s| assert!(matches!(res.board[5][s], None)));
        assert!(matches!(res.board[6][4], None));
        assert!(matches!(res.board[7][1], None));
        assert!(matches!(res.board[7][4], None));
        assert!(matches!(res.board[7][7], None));
    }

    #[test]
    fn from_fen_with_moves_and_takes() {
        let fen = "rnbk1b1r/ppNq1ppp/8/1B6/3Pp3/5N2/PPP2PPP/R1BQK2R b KQ d3 0 8";
        let res = Game::from_fen(fen);

        assert_eq!(res.active_color.to_string(), String::from("b"));

        assert_eq!(res.half_move_clock, 0);
        assert_eq!(res.full_move_number, 8);

        assert_eq!(res.white_can_castle_kingside, true);
        assert_eq!(res.white_can_castle_queenside, true);
        assert_eq!(res.black_can_castle_kingside, false);
        assert_eq!(res.black_can_castle_queenside, false);

        assert!(matches!(res.en_passant_target, Some(Square::D3)));

        // Rooks
        assert!(matches!(res.board[0][0], Some(Piece::Rook(Color::Black))));
        assert!(matches!(res.board[0][7], Some(Piece::Rook(Color::Black))));
        assert!(matches!(res.board[7][0], Some(Piece::Rook(Color::White))));
        assert!(matches!(res.board[7][7], Some(Piece::Rook(Color::White))));

        // Knights
        assert!(matches!(res.board[0][1], Some(Piece::Knight(Color::Black))));
        assert!(matches!(res.board[1][2], Some(Piece::Knight(Color::White))));
        assert!(matches!(res.board[5][5], Some(Piece::Knight(Color::White))));

        // Bishops
        assert!(matches!(res.board[0][2], Some(Piece::Bishop(Color::Black))));
        assert!(matches!(res.board[0][5], Some(Piece::Bishop(Color::Black))));
        assert!(matches!(res.board[3][1], Some(Piece::Bishop(Color::White))));
        assert!(matches!(res.board[7][2], Some(Piece::Bishop(Color::White))));

        // Queens
        assert!(matches!(res.board[1][3], Some(Piece::Queen(Color::Black))));
        assert!(matches!(res.board[7][3], Some(Piece::Queen(Color::White))));

        // Kings
        assert!(matches!(res.board[0][3], Some(Piece::King(Color::Black))));
        assert!(matches!(res.board[7][4], Some(Piece::King(Color::White))));

        // Pawns
        assert!(matches!(res.board[1][0], Some(Piece::Pawn(Color::Black))));
        assert!(matches!(res.board[1][1], Some(Piece::Pawn(Color::Black))));
        assert!(matches!(res.board[4][4], Some(Piece::Pawn(Color::Black))));
        assert!(matches!(res.board[1][5], Some(Piece::Pawn(Color::Black))));
        assert!(matches!(res.board[1][6], Some(Piece::Pawn(Color::Black))));
        assert!(matches!(res.board[1][7], Some(Piece::Pawn(Color::Black))));
        assert!(matches!(res.board[6][0], Some(Piece::Pawn(Color::White))));
        assert!(matches!(res.board[6][1], Some(Piece::Pawn(Color::White))));
        assert!(matches!(res.board[6][2], Some(Piece::Pawn(Color::White))));
        assert!(matches!(res.board[4][3], Some(Piece::Pawn(Color::White))));
        assert!(matches!(res.board[6][5], Some(Piece::Pawn(Color::White))));
        assert!(matches!(res.board[6][6], Some(Piece::Pawn(Color::White))));
        assert!(matches!(res.board[6][7], Some(Piece::Pawn(Color::White))));

        // None
        assert!(matches!(res.board[0][4], None));
        assert!(matches!(res.board[0][6], None));
        assert!(matches!(res.board[1][4], None));
        (0..8).for_each(|s| assert!(matches!(res.board[2][s], None)));
        vec![0, 2, 3, 4, 5, 6, 7]
            .iter()
            .for_each(|&s| assert!(matches!(res.board[3][s], None)));
        vec![0, 1, 2, 5, 6, 7]
            .iter()
            .for_each(|&s| assert!(matches!(res.board[4][s], None)));
        vec![0, 1, 2, 3, 4, 6, 7]
            .iter()
            .for_each(|&s| assert!(matches!(res.board[5][s], None)));
        assert!(matches!(res.board[6][3], None));
        assert!(matches!(res.board[6][4], None));
        assert!(matches!(res.board[7][1], None));
        assert!(matches!(res.board[7][5], None));
        assert!(matches!(res.board[7][6], None));
    }

    #[test]
    #[should_panic]
    fn from_fen_no_piece_data() {
        let fen = "w KQkq - 0 1";
        Game::from_fen(fen);
    }

    #[test]
    #[should_panic]
    fn from_fen_invalid_piece_data() {
        let fen = "Xnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        Game::from_fen(fen);
    }

    #[test]
    #[should_panic]
    fn from_fen_invalid_number_in_piece_data() {
        let fen = "Xnbqkbnr/pppppppp/9/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        Game::from_fen(fen);
    }

    #[test]
    #[should_panic]
    fn from_fen_no_en_passant_target() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq 0 1";
        Game::from_fen(fen);
    }

    #[test]
    #[should_panic]
    fn from_fen_no_castling() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - 0 1";
        Game::from_fen(fen);
    }

    #[test]
    #[should_panic]
    fn from_fen_no_active_color() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR KQkq - 0 1";
        Game::from_fen(fen);
    }

    #[test]
    #[should_panic]
    fn from_fen_no_half_move_clock() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 1";
        Game::from_fen(fen);
    }

    #[test]
    #[should_panic]
    fn from_fen_no_full_move_number() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0";
        Game::from_fen(fen);
    }
}
