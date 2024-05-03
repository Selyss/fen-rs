use crate::board::BoardState;
use crate::pieces::{Color, Piece, PieceType};

pub trait ToFen {
    fn to_fen(board_state: &BoardState) -> String;
    fn castling_to_fen(
        white_kingside: bool,
        white_queenside: bool,
        black_kingside: bool,
        black_queenside: bool,
    ) -> String;
    fn piece_placement_to_fen(piece_placement: &[Option<Piece>]) -> String;
    fn piece_to_fen(piece: &Piece) -> String;
    fn en_passant_to_fen(target: Option<u8>) -> String;
}

impl ToFen for BoardState {
    fn to_fen(board_state: &BoardState) -> String {
        let mut fen = String::new();
        fen.push_str(&BoardState::piece_placement_to_fen(
            &board_state.piece_placement,
        ));
        let active_color = match board_state.active_color {
            Color::White => 'w',
            Color::Black => 'b',
        };
        fen.push(' ');
        fen.push(active_color);

        fen.push(' ');
        fen.push_str(&BoardState::castling_to_fen(
            board_state.white_castle_kingside,
            board_state.white_castle_queenside,
            board_state.black_castle_kingside,
            board_state.black_castle_queenside,
        ));

        fen.push(' ');
        fen.push_str(&BoardState::en_passant_to_fen(
            board_state.en_passant_target,
        ));

        fen.push(' ');
        fen.push_str(&board_state.halfmove_clock.to_string());

        fen.push(' ');
        fen.push_str(&board_state.fullmove_clock.to_string());

        return fen;
    }
    fn piece_placement_to_fen(piece_placement: &[Option<Piece>]) -> String {
        let mut fen = String::new();
        // amount of empty squares
        let mut empty = 0;

        for (index, square) in piece_placement.iter().enumerate() {
            match square {
                Some(piece) => {
                    if empty > 0 {
                        fen.push_str(&empty.to_string());
                        empty = 0;
                    }
                    fen.push_str(&BoardState::piece_to_fen(piece));
                }
                None => empty += 1,
            }
            if (index + 1) % 8 == 0 {
                if empty > 0 {
                    fen.push_str(&empty.to_string());
                    empty = 0;
                }
                if index != 63 {
                    fen.push('/');
                }
            }
        }
        return fen;
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
    fn en_passant_to_fen(target: Option<u8>) -> String {
        match target {
            Some(square) => {
                let file = (b'a' + (square % 8) as u8) as char;
                let rank = (b'1' + (square / 8) as u8) as char;
                format!("{}{}", file, rank)
            }
            None => "-".to_string(),
        }
    }
}
