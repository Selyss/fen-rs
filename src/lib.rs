use std::fmt::Error;

/// Piece options and color piece belongs to

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Piece {
    color: Color,
    piece_type: PieceType,
}

/// The state of the game
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Board {
    // For each square, a piece can either exist or be None
    pub piece_placement: Vec<Option<Piece>>,
    pub active_color: Color,
    // most straightfoward way to handle castling is with specific fields
    pub white_castle_kingside: bool,
    pub white_castle_queenside: bool,

    pub black_castle_kingside: bool,
    pub black_castle_queenside: bool,

    // will be a single square 1-64
    pub en_passant_target: Option<u8>,

    // no way this has to be u64
    pub halfmove_clock: u32,
    pub fullmove_clock: u32,
}

impl Board {
    pub fn from_fen(fen: &str) -> Result<Board, Error> {
        let fields: Vec<_> = fen.split(" ").collect();
        let board = fields[0];
        let board: Vec<_> = board.split("/").collect();
        let mut new_board = Vec::new();
        for file in board {
            for square in file.chars() {
                new_board.push(Self::get_piece(square));
            }
        }
        println!("{:?}", new_board);

        todo!()
    }

    fn get_piece(piece: char) -> Option<Piece> {
        if piece.to_digit(10).is_some() {
            return None;
        }
        match piece {
            // White pieces
            'P' => Some(Piece {
                color: Color::White,
                piece_type: PieceType::Pawn,
            }),
            'N' => Some(Piece {
                color: Color::White,
                piece_type: PieceType::Knight,
            }),
            'B' => Some(Piece {
                color: Color::White,
                piece_type: PieceType::Bishop,
            }),
            'R' => Some(Piece {
                color: Color::White,
                piece_type: PieceType::Rook,
            }),
            'Q' => Some(Piece {
                color: Color::White,
                piece_type: PieceType::Queen,
            }),
            'K' => Some(Piece {
                color: Color::White,
                piece_type: PieceType::King,
            }),

            // Black pieces
            'p' => Some(Piece {
                color: Color::Black,
                piece_type: PieceType::Pawn,
            }),
            'n' => Some(Piece {
                color: Color::Black,
                piece_type: PieceType::Knight,
            }),
            'b' => Some(Piece {
                color: Color::Black,
                piece_type: PieceType::Bishop,
            }),
            'r' => Some(Piece {
                color: Color::Black,
                piece_type: PieceType::Rook,
            }),
            'q' => Some(Piece {
                color: Color::Black,
                piece_type: PieceType::Queen,
            }),
            'k' => Some(Piece {
                color: Color::Black,
                piece_type: PieceType::King,
            }),
            _ => panic!("Error: failed to parse piece"),
        }
    }
}
