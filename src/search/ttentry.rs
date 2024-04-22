use crate::chess::mv::Mv;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum Flag {
    #[default]
    Exact = 0,
    Lower,
    Upper,
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct TTEntry {
    pub hash: u64,
    pub mv: Mv,
    pub score: i32,
    pub depth: i32,
    pub flag: Flag,
}
