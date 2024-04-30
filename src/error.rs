use std::fmt;

#[derive(Debug)]
pub enum FenError {
    NotEnoughFields(String),
    BadPieceNotation(String),
    BadActiveColor(String),
    BadEnPassant(String),
    BadHalfmove(String),
    BadFullmove(String),
}

impl fmt::Display for FenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FenError::NotEnoughFields(msg) => write!(f, "Not enough FEN fields: {}", msg),
            FenError::BadPieceNotation(msg) => write!(f, "Piece notation error: {}", msg),
            FenError::BadActiveColor(msg) => write!(f, "Active color error: {}", msg),
            FenError::BadEnPassant(msg) => write!(f, "En passant error: {}", msg),
            FenError::BadHalfmove(msg) => write!(f, "Halfmove error: {}", msg),
            FenError::BadFullmove(msg) => write!(f, "Fullmove error: {}", msg),
        }
    }
}

impl std::error::Error for FenError {}
