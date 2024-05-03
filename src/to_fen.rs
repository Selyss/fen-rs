use crate::BoardState;

pub trait ToFen {
    fn to_fen(&self) -> String;
}

impl ToFen for BoardState {
    fn to_fen(&self) -> String {
        let mut fen = String::new();
        todo!()
    }
}
