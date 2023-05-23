use std::str::Chars;

mod engine;

pub enum Color {
    White,
    Black,
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

impl Game {
    pub fn new() -> Game {
        Game {
            board: [
                [
                    Some(Piece::Rook(Color::Black)),
                    Some(Piece::Knight(Color::Black)),
                    Some(Piece::Bishop(Color::Black)),
                    Some(Piece::Queen(Color::Black)),
                    Some(Piece::King(Color::Black)),
                    Some(Piece::Bishop(Color::Black)),
                    Some(Piece::Knight(Color::Black)),
                    Some(Piece::Rook(Color::Black)),
                ],
                [
                    Some(Piece::Pawn(Color::Black)),
                    Some(Piece::Pawn(Color::Black)),
                    Some(Piece::Pawn(Color::Black)),
                    Some(Piece::Pawn(Color::Black)),
                    Some(Piece::Pawn(Color::Black)),
                    Some(Piece::Pawn(Color::Black)),
                    Some(Piece::Pawn(Color::Black)),
                    Some(Piece::Pawn(Color::Black)),
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    Some(Piece::Pawn(Color::White)),
                    Some(Piece::Pawn(Color::White)),
                    Some(Piece::Pawn(Color::White)),
                    Some(Piece::Pawn(Color::White)),
                    Some(Piece::Pawn(Color::White)),
                    Some(Piece::Pawn(Color::White)),
                    Some(Piece::Pawn(Color::White)),
                    Some(Piece::Pawn(Color::White)),
                ],
                [
                    Some(Piece::Rook(Color::White)),
                    Some(Piece::Knight(Color::White)),
                    Some(Piece::Bishop(Color::White)),
                    Some(Piece::Queen(Color::White)),
                    Some(Piece::King(Color::White)),
                    Some(Piece::Bishop(Color::White)),
                    Some(Piece::Knight(Color::White)),
                    Some(Piece::Rook(Color::White)),
                ],
            ],
            active_color: Color::White,
            white_can_castle_kingside: true,
            white_can_castle_queenside: true,
            black_can_castle_kingside: true,
            black_can_castle_queenside: true,
            en_passant_target: None,
            half_move_clock: 0,
            full_move_number: 1,
        }
    }

    pub fn from_fen(&self, fen: &str) -> Game {
        let fen = String::from(fen);
        let mut pieces = fen.split_whitespace();
        let rows = pieces
            .next()
            .expect("Incorrectly formatted string")
            .split("/");
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
        for (row_index, row) in rows.enumerate() {
            for (piece_index, piece) in row.chars().enumerate() {
                let board_position = match piece {
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
                    '1' => None,
                    '2' => None,
                    '3' => None,
                    '4' => None,
                    '5' => None,
                    '6' => None,
                    '7' => None,
                    '8' => None,
                    _ => panic!("Invalid character entered in piece placement data."),
                };
                board[row_index][piece_index] = board_position;
            }
        }

        let active_color = match pieces.next() {
            Some("w") => Color::White,
            Some("b") => Color::Black,
            None => panic!("Must have active color in FEN"),
            _ => panic!("Must have active color in FEN"),
        };

        let mut castleable = pieces.next().expect("Required").chars();
        let white_can_castle_kingside = is_castleable(
            &mut castleable,
            "Must provide castle info for White King Side",
        );
        let white_can_castle_queenside = is_castleable(
            &mut castleable,
            "Must provide castle info for Black Queen Side",
        );
        let black_can_castle_kingside = is_castleable(
            &mut castleable,
            "Must provide castle info for Black King Side",
        );
        let black_can_castle_queenside = is_castleable(
            &mut castleable,
            "Must provide castle info for Black Queen Side",
        );

        let target_square = pieces.next();
        let en_passant_target = convert_en_passant_target(target_square);

        let half_move_clock = pieces
            .next()
            .expect("Must provide half move clock value in FEN")
            .parse::<u8>()
            .expect("Could not parse half move clock value");

        let full_move_number = pieces
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
        let pieces = game_board_to_piece_str(&self.board);
        let active_color = match self.active_color {
            Color::White => String::from("w"),
            Color::Black => String::from("b"),
        };

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

        let en_passant_square = square_to_string(&self.en_passant_target);
        let half_move = &self.half_move_clock.to_string();
        let full_move = &self.full_move_number.to_string();

        format!("{pieces} {active_color} {castleable} {en_passant_square} {half_move} {full_move}")
    }

    pub fn make_move(&mut self, from: Square, to: Square) -> bool {
        true
    }
}

fn square_to_string(square: &Option<Square>) -> &str {
    match square {
        Some(Square::A1) => "a1",
        Some(Square::A2) => "a2",
        Some(Square::A3) => "a3",
        Some(Square::A4) => "a4",
        Some(Square::A5) => "a5",
        Some(Square::A6) => "a6",
        Some(Square::A7) => "a7",
        Some(Square::A8) => "a8",
        Some(Square::B1) => "b1",
        Some(Square::B2) => "b2",
        Some(Square::B3) => "b3",
        Some(Square::B4) => "b4",
        Some(Square::B5) => "b5",
        Some(Square::B6) => "b6",
        Some(Square::B7) => "b7",
        Some(Square::B8) => "b8",
        Some(Square::C1) => "c1",
        Some(Square::C2) => "c2",
        Some(Square::C3) => "c3",
        Some(Square::C4) => "c4",
        Some(Square::C5) => "c5",
        Some(Square::C6) => "c6",
        Some(Square::C7) => "c7",
        Some(Square::C8) => "c8",
        Some(Square::D1) => "d1",
        Some(Square::D2) => "d2",
        Some(Square::D3) => "d3",
        Some(Square::D4) => "d4",
        Some(Square::D5) => "d5",
        Some(Square::D6) => "d6",
        Some(Square::D7) => "d7",
        Some(Square::D8) => "d8",
        Some(Square::E1) => "e1",
        Some(Square::E2) => "e2",
        Some(Square::E3) => "e3",
        Some(Square::E4) => "e4",
        Some(Square::E5) => "e5",
        Some(Square::E6) => "e6",
        Some(Square::E7) => "e7",
        Some(Square::E8) => "e8",
        Some(Square::F1) => "f1",
        Some(Square::F2) => "f2",
        Some(Square::F3) => "f3",
        Some(Square::F4) => "f4",
        Some(Square::F5) => "f5",
        Some(Square::F6) => "f6",
        Some(Square::F7) => "f7",
        Some(Square::F8) => "f8",
        Some(Square::G1) => "g1",
        Some(Square::G2) => "g2",
        Some(Square::G3) => "g3",
        Some(Square::G4) => "g4",
        Some(Square::G5) => "g5",
        Some(Square::G6) => "g6",
        Some(Square::G7) => "g7",
        Some(Square::G8) => "g8",
        Some(Square::H1) => "h1",
        Some(Square::H2) => "h2",
        Some(Square::H3) => "h3",
        Some(Square::H4) => "h4",
        Some(Square::H5) => "h5",
        Some(Square::H6) => "h6",
        Some(Square::H7) => "h7",
        Some(Square::H8) => "h8",
        None => "-",
    }
}

fn game_board_to_piece_str(board: &[[Option<Piece>; 8]; 8]) -> String {
    // Count of how many spaces in the row since last piece
    let mut num = 0;
    let mut pieces = String::new();

    for row in board {
        for piece in row {
            let p = match piece {
                Some(Piece::Pawn(Color::Black)) => "p",
                Some(Piece::Rook(Color::Black)) => "r",
                Some(Piece::Knight(Color::Black)) => "n",
                Some(Piece::Bishop(Color::Black)) => "b",
                Some(Piece::Queen(Color::Black)) => "q",
                Some(Piece::King(Color::Black)) => "k",
                Some(Piece::Pawn(Color::White)) => "P",
                Some(Piece::Rook(Color::White)) => "R",
                Some(Piece::Knight(Color::White)) => "N",
                Some(Piece::Bishop(Color::White)) => "B",
                Some(Piece::Queen(Color::White)) => "Q",
                Some(Piece::King(Color::White)) => "K",
                None => "empty",
            };
            if p != "empty" {
                if num > 0 {
                    pieces += &num.to_string();
                    num = 0;
                }
                pieces += p;
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

fn is_castleable(availability: &mut Chars, error_message: &str) -> bool {
    availability.next().expect(error_message) != '-'
}

fn convert_en_passant_target(target_square: Option<&str>) -> Option<Square> {
    match target_square {
        Some("a1") => Some(Square::A1),
        Some("a2") => Some(Square::A2),
        Some("a3") => Some(Square::A3),
        Some("a4") => Some(Square::A4),
        Some("a5") => Some(Square::A5),
        Some("a6") => Some(Square::A6),
        Some("a7") => Some(Square::A7),
        Some("a8") => Some(Square::A8),
        Some("b1") => Some(Square::B1),
        Some("b2") => Some(Square::B2),
        Some("b3") => Some(Square::B3),
        Some("b4") => Some(Square::B4),
        Some("b5") => Some(Square::B5),
        Some("b6") => Some(Square::B6),
        Some("b7") => Some(Square::B7),
        Some("b8") => Some(Square::B8),
        Some("c1") => Some(Square::C1),
        Some("c2") => Some(Square::C2),
        Some("c3") => Some(Square::C3),
        Some("c4") => Some(Square::C4),
        Some("c5") => Some(Square::C5),
        Some("c6") => Some(Square::C6),
        Some("c7") => Some(Square::C7),
        Some("c8") => Some(Square::C8),
        Some("d1") => Some(Square::D1),
        Some("d2") => Some(Square::D2),
        Some("d3") => Some(Square::D3),
        Some("d4") => Some(Square::D4),
        Some("d5") => Some(Square::D5),
        Some("d6") => Some(Square::D6),
        Some("d7") => Some(Square::D7),
        Some("d8") => Some(Square::D8),
        Some("e1") => Some(Square::E1),
        Some("e2") => Some(Square::E2),
        Some("e3") => Some(Square::E3),
        Some("e4") => Some(Square::E4),
        Some("e5") => Some(Square::E5),
        Some("e6") => Some(Square::E6),
        Some("e7") => Some(Square::E7),
        Some("e8") => Some(Square::E8),
        Some("f1") => Some(Square::F1),
        Some("f2") => Some(Square::F2),
        Some("f3") => Some(Square::F3),
        Some("f4") => Some(Square::F4),
        Some("f5") => Some(Square::F5),
        Some("f6") => Some(Square::F6),
        Some("f7") => Some(Square::F7),
        Some("f8") => Some(Square::F8),
        Some("g1") => Some(Square::G1),
        Some("g2") => Some(Square::G2),
        Some("g3") => Some(Square::G3),
        Some("g4") => Some(Square::G4),
        Some("g5") => Some(Square::G5),
        Some("g6") => Some(Square::G6),
        Some("g7") => Some(Square::G7),
        Some("g8") => Some(Square::G8),
        Some("h1") => Some(Square::H1),
        Some("h2") => Some(Square::H2),
        Some("h3") => Some(Square::H3),
        Some("h4") => Some(Square::H4),
        Some("h5") => Some(Square::H5),
        Some("h6") => Some(Square::H6),
        Some("h7") => Some(Square::H7),
        Some("h8") => Some(Square::H8),
        Some("-") => None,
        None => panic!("Must provide en passant target in FEN"),
        _ => panic!("Must provide en passant target in FEN"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_fen_partial() {
        let game = Game::new();

        let f = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert_eq!(f, game.to_fen());
    }

    #[test]
    fn from_fen_success() {
        // New game fen string
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let game = Game::new();
        let res = game.from_fen(fen);
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
    fn to_fen_success() {
        let game = Game::new();

        assert_eq!(
            game.to_fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
    }
}
