use crate::{chess::mv::Mv, chess::position::Position};

pub struct Info {
    pub pos: Position,
    pub depth: Option<i32>,
    pub seldepth: Option<i32>,
    pub nodes: Option<u64>,
    pub score: Option<i32>,
    pub mate: Option<i32>,
    pub elapsed: Option<u128>,
    pub pv: Vec<Mv>,
}
