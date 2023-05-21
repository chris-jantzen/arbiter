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

    pub fn from_fen(fen: &str) -> Option<Game> {
        todo!()
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        // Count of how many spaces in the row since last piece
        let mut num = 0;
        let mut pieces = String::new();

        for row in &self.board {
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
        fen = pieces[..pieces.len() - 1].to_string();

        fen += " ";

        fen += match self.active_color {
            Color::White => "w",
            Color::Black => "b",
        };

        fen += " ";

        if self.white_can_castle_kingside {
            fen += "K";
        }
        if self.white_can_castle_queenside {
            fen += "Q";
        }
        if self.black_can_castle_kingside {
            fen += "k";
        }
        if self.black_can_castle_queenside {
            fen += "q";
        }

        fen += " ";
        fen += match self.en_passant_target {
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
        };

        fen += " ";
        fen += &self.half_move_clock.to_string()[..];

        fen += " ";
        fen += &self.full_move_number.to_string()[..];

        return fen;
    }

    pub fn make_move(&mut self, from: Square, to: Square) -> bool {
        true
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
        let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert_eq!(fen, fen.clone()); // TEMP
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
