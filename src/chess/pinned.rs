use super::{bitboard::Bitboard, position::Position, rays, side::Side, square::Square};

impl Position {
    #[must_use]
    pub fn pinned_rook(&self, sq: Square, _side: Side) -> (Bitboard, Bitboard) {
        let mut pinned = Bitboard::empty();
        let mut xrays = Bitboard::empty();

        let rq = self.get_them() & (self.get_rooks() | self.get_queens());

        let north = rays::ray_n(sq, self.get_occupied());
        if (north & self.get_us()).is_occupied() {
            let bb = north & self.get_us();
            let xray = rays::ray_n(sq, self.get_occupied() ^ bb);
            if (xray & rq).is_occupied() {
                pinned |= bb;
                xrays |= xray;
            }
        }

        let east = rays::ray_e(sq, self.get_occupied());
        if (east & self.get_us()).is_occupied() {
            let bb = east & self.get_us();
            let xray = rays::ray_e(sq, self.get_occupied() ^ bb);
            if (xray & rq).is_occupied() {
                pinned |= bb;
                xrays |= xray;
            }
        }

        let south = rays::ray_s(sq, self.get_occupied());
        if (south & self.get_us()).is_occupied() {
            let bb = south & self.get_us();
            let xray = rays::ray_s(sq, self.get_occupied() ^ bb);
            if (xray & rq).is_occupied() {
                pinned |= bb;
                xrays |= xray;
            }
        }

        let west = rays::ray_w(sq, self.get_occupied());
        if (west & self.get_us()).is_occupied() {
            let bb = west & self.get_us();
            let xray = rays::ray_w(sq, self.get_occupied() ^ bb);
            if (xray & rq).is_occupied() {
                pinned |= bb;
                xrays |= xray;
            }
        }

        (pinned, xrays)
    }

    pub fn pinned_bishop(&self, sq: Square, _side: Side) -> (Bitboard, Bitboard) {
        let mut pinned = Bitboard::empty();
        let mut xrays = Bitboard::empty();

        let bq = self.get_them() & (self.get_bishops() | self.get_queens());

        let nw = rays::ray_nw(sq, self.get_occupied());
        if (nw & self.get_us()).is_occupied() {
            let bb = nw & self.get_us();
            let xray = rays::ray_nw(sq, self.get_occupied() ^ bb);
            if (xray & bq).is_occupied() {
                pinned |= bb;
                xrays |= xray;
            }
        }

        let ne = rays::ray_ne(sq, self.get_occupied());
        if (ne & self.get_us()).is_occupied() {
            let bb = ne & self.get_us();
            let xray = rays::ray_ne(sq, self.get_occupied() ^ bb);
            if (xray & bq).is_occupied() {
                pinned |= bb;
                xrays |= xray;
            }
        }

        let sw = rays::ray_sw(sq, self.get_occupied());
        if (sw & self.get_us()).is_occupied() {
            let bb = sw & self.get_us();
            let xray = rays::ray_sw(sq, self.get_occupied() ^ bb);
            if (xray & bq).is_occupied() {
                pinned |= bb;
                xrays |= xray;
            }
        }

        let se = rays::ray_se(sq, self.get_occupied());
        if (se & self.get_us()).is_occupied() {
            let bb = se & self.get_us();
            let xray = rays::ray_se(sq, self.get_occupied() ^ bb);
            if (xray & bq).is_occupied() {
                pinned |= bb;
                xrays |= xray;
            }
        }

        (pinned, xrays)
    }

    #[must_use]
    pub fn pinned(&self, sq: Square, side: Side) -> (Bitboard, Bitboard) {
        let (bpinned, bxrays) = self.pinned_bishop(sq, side);
        let (rpinned, rxrays) = self.pinned_rook(sq, side);
        (bpinned | rpinned, bxrays | rxrays)
    }
}

#[cfg(test)]
mod tests {
    use crate::chess::{bitboard::Bitboard, position::Position, side::Side};

    #[test]
    fn pinned() {
        let tests = [
            ("startpos", Bitboard::empty(), Bitboard::empty()),
            (
                "k7/4r3/8/4N3/1r1NKN1r/4N3/8/4r3 w - - 0 1",
                Bitboard(0x1028100000),
                Bitboard(0x101010ee101010),
            ),
            (
                "k7/4q3/8/4N3/1q1NKN1q/4N3/8/4q3 w - - 0 1",
                Bitboard(0x1028100000),
                Bitboard(0x101010ee101010),
            ),
        ];

        for (fen, pinned, xrayed) in tests {
            let pos = Position::from_fen(&fen);
            let ksq = (pos.get_us() & pos.get_kings()).lsb();
            let (pins, xrays) = pos.pinned_rook(ksq, Side::Them);
            assert_eq!(pins, pinned);
            assert_eq!(xrays, xrayed);
        }
    }
}
