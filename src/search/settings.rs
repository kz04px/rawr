pub enum Type {
    Time(u32, u32, Option<u32>, Option<u32>, Option<u32>),
    Movetime(u32),
    Depth(i32),
    Nodes(u64),
    Infinite,
    Perft(u8),
    SplitPerft(u8),
}
