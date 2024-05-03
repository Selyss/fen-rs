use crate::BoardState;
use crate::Color;
use crate::Piece;
use crate::PieceType;

pub trait ToFen {
    fn to_fen(&self) -> String;
    fn castling_to_fen(
        white_kingside: bool,
        white_queenside: bool,
        black_kingside: bool,
        black_queenside: bool,
    ) -> String;
    fn piece_to_fen(piece: &Piece) -> String;
}

impl ToFen for BoardState {
    fn to_fen(&self) -> String {
        let mut fen = String::new();
        todo!()
    }
    fn piece_to_fen(piece: &Piece) -> String {
        match piece.piece_type {
            PieceType::Pawn => match piece.color {
                Color::White => "P".to_string(),
                Color::Black => "p".to_string(),
            },
            PieceType::Knight => match piece.color {
                Color::White => "N".to_string(),
                Color::Black => "n".to_string(),
            },
            PieceType::Bishop => match piece.color {
                Color::White => "B".to_string(),
                Color::Black => "b".to_string(),
            },
            PieceType::Rook => match piece.color {
                Color::White => "R".to_string(),
                Color::Black => "r".to_string(),
            },
            PieceType::Queen => match piece.color {
                Color::White => "Q".to_string(),
                Color::Black => "q".to_string(),
            },
            PieceType::King => match piece.color {
                Color::White => "K".to_string(),
                Color::Black => "k".to_string(),
            },
        }
    }
    fn castling_to_fen(
        white_kingside: bool,
        white_queenside: bool,
        black_kingside: bool,
        black_queenside: bool,
    ) -> String {
        let mut fen = String::new();
        if white_kingside {
            fen.push('K');
        }
        if white_queenside {
            fen.push('Q');
        }
        if black_kingside {
            fen.push('k');
        }
        if black_queenside {
            fen.push('q');
        }
        if fen.is_empty() {
            fen.push('-');
        }
        return fen;
    }
}
