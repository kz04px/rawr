use crate::chess::position::Position;

impl Position {
    pub fn flip(&mut self) {
        // Flip
        self.colours[0] = self.colours[0].flip();
        self.colours[1] = self.colours[1].flip();
        self.pieces[0] = self.pieces[0].flip();
        self.pieces[1] = self.pieces[1].flip();
        self.pieces[2] = self.pieces[2].flip();
        self.pieces[3] = self.pieces[3].flip();
        self.pieces[4] = self.pieces[4].flip();
        self.pieces[5] = self.pieces[5].flip();

        // Swap colours
        (self.colours[0], self.colours[1]) = (self.colours[1], self.colours[0]);

        // Swap castling permissions
        (self.us_ksc, self.them_ksc) = (self.them_ksc, self.us_ksc);
        (self.us_qsc, self.them_qsc) = (self.them_qsc, self.us_qsc);

        if self.ep.is_some() {
            self.ep = Some(self.ep.unwrap().flip());
        }

        self.turn = !self.turn;
    }
}
