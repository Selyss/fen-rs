mod error;

use std::fmt::Error;

use error::FenError;

pub const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

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
pub struct BoardState {
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

impl BoardState {
    pub fn new() -> BoardState {
        let fields: Vec<_> = STARTING_FEN.split(" ").collect();
        let starting_board = Self::create_board(fields[0]);
        return BoardState {
            piece_placement: starting_board,
            active_color: Color::White,
            white_castle_kingside: true,
            white_castle_queenside: true,
            black_castle_kingside: true,
            black_castle_queenside: true,
            en_passant_target: None,
            halfmove_clock: 0,
            fullmove_clock: 1,
        };
    }

    pub fn from_fen(fen: &str) -> Result<BoardState, FenError> {
        let mut fen_struct = BoardState::new();
        let fields: Vec<_> = fen.split(" ").collect();
        let board = Self::create_board(fields[0]);
        fen_struct.piece_placement = board;

        let active_color = match Self::get_active_color(fields[1]) {
            Ok(color) => color,
            Err(error) => panic!("{}", error),
        };
        fen_struct.active_color = active_color;

        let castling = fields[2];
        // maybe extract into function later
        fen_struct.white_castle_kingside = castling.contains('K');
        fen_struct.white_castle_queenside = castling.contains('Q');
        fen_struct.black_castle_kingside = castling.contains('k');
        fen_struct.black_castle_queenside = castling.contains('q');

        let en_passant = match fields[3] {
            // TODO:
            _ => None,
        };

        fen_struct.en_passant_target = en_passant;

        let halfmove = fields[4]
            .parse::<u32>()
            .expect("Error: failed to parse halfmove clock");

        fen_struct.halfmove_clock = halfmove;

        let fullmove = fields[5]
            .parse::<u32>()
            .expect("Error: failed to parse fullmove number");

        fen_struct.fullmove_clock = fullmove;

        println!("{:?}", fen_struct);

        return Ok(fen_struct);
    }

    fn get_active_color(field: &str) -> Result<Color, FenError> {
        match field {
            "w" => Ok(Color::White),
            "b" => Ok(Color::White),
            _ => Err(FenError::BadActiveColor(("Unknown Color").to_string())),
        }
    }

    // board portion of the fen
    fn create_board(board: &str) -> Vec<Option<Piece>> {
        let board: Vec<_> = board.split("/").collect();
        let mut new_board = Vec::new();
        for file in board {
            for sq in file.chars() {
                let piece = match Self::get_piece(sq) {
                    Ok(square) => square,
                    Err(error) => panic!("{}", error),
                };
                new_board.push(piece);
            }
        }
        return new_board;
    }

    fn get_piece(piece: char) -> Result<Option<Piece>, FenError> {
        if piece.to_digit(10).is_some() {
            return Ok(None);
        }
        match piece {
            // White pieces
            'P' => Ok(Some(Piece {
                color: Color::White,
                piece_type: PieceType::Pawn,
            })),
            'N' => Ok(Some(Piece {
                color: Color::White,
                piece_type: PieceType::Knight,
            })),
            'B' => Ok(Some(Piece {
                color: Color::White,
                piece_type: PieceType::Bishop,
            })),
            'R' => Ok(Some(Piece {
                color: Color::White,
                piece_type: PieceType::Rook,
            })),
            'Q' => Ok(Some(Piece {
                color: Color::White,
                piece_type: PieceType::Queen,
            })),
            'K' => Ok(Some(Piece {
                color: Color::White,
                piece_type: PieceType::King,
            })),

            // Black pieces
            'p' => Ok(Some(Piece {
                color: Color::Black,
                piece_type: PieceType::Pawn,
            })),
            'n' => Ok(Some(Piece {
                color: Color::Black,
                piece_type: PieceType::Knight,
            })),
            'b' => Ok(Some(Piece {
                color: Color::Black,
                piece_type: PieceType::Bishop,
            })),
            'r' => Ok(Some(Piece {
                color: Color::Black,
                piece_type: PieceType::Rook,
            })),
            'q' => Ok(Some(Piece {
                color: Color::Black,
                piece_type: PieceType::Queen,
            })),
            'k' => Ok(Some(Piece {
                color: Color::Black,
                piece_type: PieceType::King,
            })),
            _ => Err(FenError::BadPieceNotation(("Unknown piece").to_string())),
        }
    }
    // TODO:
    pub fn to_fen(board_state: BoardState) -> String {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Color::{Black, White};
    use crate::PieceType::{Bishop, King, Knight, Pawn, Queen, Rook};

    #[test]
    fn test_starting_position() {
        let start_fen = BoardState::from_fen(STARTING_FEN).expect("Error");

        let start_board = BoardState {
            piece_placement: [
                Some(Piece {
                    color: Black,
                    piece_type: Rook,
                }),
                Some(Piece {
                    color: Black,
                    piece_type: Knight,
                }),
                Some(Piece {
                    color: Black,
                    piece_type: Bishop,
                }),
                Some(Piece {
                    color: Black,
                    piece_type: Queen,
                }),
                Some(Piece {
                    color: Black,
                    piece_type: King,
                }),
                Some(Piece {
                    color: Black,
                    piece_type: Bishop,
                }),
                Some(Piece {
                    color: Black,
                    piece_type: Knight,
                }),
                Some(Piece {
                    color: Black,
                    piece_type: Rook,
                }),
                Some(Piece {
                    color: Black,
                    piece_type: Pawn,
                }),
                Some(Piece {
                    color: Black,
                    piece_type: Pawn,
                }),
                Some(Piece {
                    color: Black,
                    piece_type: Pawn,
                }),
                Some(Piece {
                    color: Black,
                    piece_type: Pawn,
                }),
                Some(Piece {
                    color: Black,
                    piece_type: Pawn,
                }),
                Some(Piece {
                    color: Black,
                    piece_type: Pawn,
                }),
                Some(Piece {
                    color: Black,
                    piece_type: Pawn,
                }),
                Some(Piece {
                    color: Black,
                    piece_type: Pawn,
                }),
                None,
                None,
                None,
                None,
                Some(Piece {
                    color: White,
                    piece_type: Pawn,
                }),
                Some(Piece {
                    color: White,
                    piece_type: Pawn,
                }),
                Some(Piece {
                    color: White,
                    piece_type: Pawn,
                }),
                Some(Piece {
                    color: White,
                    piece_type: Pawn,
                }),
                Some(Piece {
                    color: White,
                    piece_type: Pawn,
                }),
                Some(Piece {
                    color: White,
                    piece_type: Pawn,
                }),
                Some(Piece {
                    color: White,
                    piece_type: Pawn,
                }),
                Some(Piece {
                    color: White,
                    piece_type: Pawn,
                }),
                Some(Piece {
                    color: White,
                    piece_type: Rook,
                }),
                Some(Piece {
                    color: White,
                    piece_type: Knight,
                }),
                Some(Piece {
                    color: White,
                    piece_type: Bishop,
                }),
                Some(Piece {
                    color: White,
                    piece_type: Queen,
                }),
                Some(Piece {
                    color: White,
                    piece_type: King,
                }),
                Some(Piece {
                    color: White,
                    piece_type: Bishop,
                }),
                Some(Piece {
                    color: White,
                    piece_type: Knight,
                }),
                Some(Piece {
                    color: White,
                    piece_type: Rook,
                }),
            ]
            .to_vec(),
            active_color: White,
            white_castle_kingside: true,
            white_castle_queenside: true,
            black_castle_kingside: true,
            black_castle_queenside: true,
            en_passant_target: None,
            halfmove_clock: 0,
            fullmove_clock: 1,
        };
    }
}
