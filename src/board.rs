use crate::error::FenError;
use crate::pieces::{Color, Piece, PieceType};
use serde::{Deserialize, Serialize};
use serde_json;

pub const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
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

    pub fn create_board(board: &str) -> Result<Vec<Option<Piece>>, FenError> {
        let mut piece_placement = Vec::new();
        for file in board.split('/') {
            for char in file.chars() {
                if let Some(digit) = char.to_digit(10) {
                    for _ in 0..digit {
                        piece_placement.push(None);
                    }
                } else {
                    piece_placement.push(Self::get_piece(char)?);
                }
            }
        }
        Ok(piece_placement)
    }

    fn get_piece(piece: char) -> Result<Option<Piece>, FenError> {
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

    // should these modify or make a new one
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        return serde_json::to_string(&self);
    }

    pub fn from_json(json_str: &str) -> Result<BoardState, serde_json::Error> {
        return serde_json::from_str(json_str);
    }
}
