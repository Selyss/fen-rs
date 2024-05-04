pub mod board;
pub mod error;
pub mod pieces;
pub mod to_fen;

#[cfg(test)]
mod test {
    use crate::{
        board::{BoardState, STARTING_FEN},
        pieces::{Color, Piece, PieceType},
        to_fen::ToFen,
    };

    #[test]
    fn test_create_board() {
        let fields: Vec<_> = STARTING_FEN.split(" ").collect();
        let board = BoardState::create_board(fields[0]).unwrap();

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

    #[test]
    fn test_to_fen() {
        let board_state = BoardState::from_fen(STARTING_FEN).unwrap();
        assert_eq!(STARTING_FEN, BoardState::to_fen(&board_state));
        let board_state =
            BoardState::from_fen("8/5pkp/4b3/p3P1b1/1p5P/1P1R2B1/P5PK/2rB4 b - - 0 33").unwrap();

        assert_eq!(
            "8/5pkp/4b3/p3P1b1/1p5P/1P1R2B1/P5PK/2rB4 b - - 0 33",
            BoardState::to_fen(&board_state)
        );
    }
}
