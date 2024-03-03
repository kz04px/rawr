use crate::chess::position::Position;

impl Position {
    #[must_use]
    pub fn perft(&self, depth: u8) -> u64 {
        if depth == 0 {
            return 1;
        } else if depth == 1 {
            return self.count_moves() as u64;
        }

        let mut nodes = 0u64;

        self.move_generator(|mv| {
            let npos = self.after_move::<false>(&mv);
            nodes += npos.perft(depth - 1)
        });

        nodes
    }
}
