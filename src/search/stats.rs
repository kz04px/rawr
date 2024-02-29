use crate::chess::mv::Mv;

#[derive(Default)]
pub struct Stats {
    pub depth: i32,
    pub seldepth: i32,
    pub nodes: u64,
    pub best_move: Option<Mv>,
}
