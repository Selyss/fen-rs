mod error;

use error::FenError;

pub const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

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
    pub piece_placement: Vec<Option<Piece>>,
    pub active_color: Color,
    pub white_castle_kingside: bool,
    pub white_castle_queenside: bool,

    pub black_castle_kingside: bool,
    pub black_castle_queenside: bool,

    pub en_passant_target: Option<u8>,

    pub halfmove_clock: u32,
    pub fullmove_clock: u32,
}

impl BoardState {
    pub fn new() -> Result<BoardState, FenError> {
        let fields: Vec<_> = STARTING_FEN.split(" ").collect();
        let starting_board = match Self::create_board(fields[0]) {
            Ok(board) => board,
            Err(error) => panic!("{}", error),
        };
        return Ok(BoardState {
            piece_placement: starting_board,
            active_color: Color::White,
            white_castle_kingside: true,
            white_castle_queenside: true,
            black_castle_kingside: true,
            black_castle_queenside: true,
            en_passant_target: None,
            halfmove_clock: 0,
            fullmove_clock: 1,
        });
    }

    pub fn from_fen(fen: &str) -> Result<BoardState, FenError> {
        let mut fen_struct = BoardState::new()?;
        let fields: Vec<_> = fen.split(" ").collect();
        let board = Self::create_board(fields[0])?;
        fen_struct.piece_placement = board;

        let active_color = Self::get_active_color(fields[1])?;
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

        fen_struct.halfmove_clock = fields[4]
            .parse::<u32>()
            .map_err(|_| FenError::BadHalfmove("Failed to parse halfmove clock".to_string()))?;

        fen_struct.fullmove_clock = fields[5]
            .parse::<u32>()
            .map_err(|_| FenError::BadFullmove("Failed to parse fullmove number".to_string()))?;

        return Ok(fen_struct);
        // TODO: propegate Err to function
    }

    fn get_active_color(field: &str) -> Result<Color, FenError> {
        match field {
            "w" => Ok(Color::White),
            "b" => Ok(Color::Black),
            _ => Err(FenError::BadActiveColor("Unknown Color".to_string())),
        }
    }

    // board portion of the fen
    fn create_board(board: &str) -> Result<Vec<Option<Piece>>, FenError> {
        board
            .split('/')
            .flat_map(|file| file.chars().map(Self::get_piece))
            .collect()
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
            _ => {
                let piece_str = format!("{}", piece);
                Err(FenError::BadPieceNotation(format!(
                    "Unknown piece: '{}'",
                    piece_str
                )))
            }
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
        assert_eq!(start_fen, start_board);
    }
    #[test]
    fn test_create_board() {
        let fields: Vec<_> = STARTING_FEN.split(" ").collect();
        let board = BoardState::create_board(fields[0]).unwrap();

        println!("{:?}", board);
        assert_eq!(board.len(), 64);

        assert_eq!(
            board[0],
            Some(Piece {
                color: Color::Black,
                piece_type: PieceType::Rook
            })
        );
        assert_eq!(
            board[7],
            Some(Piece {
                color: Color::Black,
                piece_type: PieceType::Rook
            })
        );
        assert_eq!(
            board[56],
            Some(Piece {
                color: Color::White,
                piece_type: PieceType::Rook
            })
        );
        assert_eq!(
            board[63],
            Some(Piece {
                color: Color::White,
                piece_type: PieceType::Rook
            })
        );
    }

    #[test]
    fn test_from_fen() {
        let board_state = BoardState::from_fen(STARTING_FEN).unwrap();

        assert_eq!(board_state.active_color, Color::White);

        assert_eq!(board_state.white_castle_kingside, true);
        assert_eq!(board_state.white_castle_queenside, true);
        assert_eq!(board_state.black_castle_kingside, true);
        assert_eq!(board_state.black_castle_queenside, true);

        assert_eq!(board_state.halfmove_clock, 0);
        assert_eq!(board_state.fullmove_clock, 1);
    }
}
