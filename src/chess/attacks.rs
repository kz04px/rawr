use super::{bitboard::Bitboard, magic, position::Position, rays, side::Side, square::Square};

#[must_use]
pub fn is_safe(
    sq: Square,
    blockers: Bitboard,
    pawns: Bitboard,
    knights: Bitboard,
    bishops: Bitboard,
    rooks: Bitboard,
    queens: Bitboard,
    kings: Bitboard,
) -> bool {
    debug_assert!(sq.0 < 64);

    let bb = Bitboard::from_square(sq);

    // Pawns
    if rays::pawns::<false>(pawns).is_set(sq) {
        false
    }
    // Knights
    else if (rays::knights(bb) & knights).is_occupied() {
        return false;
    }
    // Bishops - Queens
    else if (magic::bishop_moves(sq.0 as i32, blockers.0) & (bishops | queens)).is_occupied() {
        return false;
    }
    // Rooks - Queens
    else if (magic::rook_moves(sq.0 as i32, blockers.0) & (rooks | queens)).is_occupied() {
        return false;
    }
    // King
    else if (rays::adjacent(sq) & kings).is_occupied() {
        return false;
    }
    // None
    else {
        return true;
    }
}

impl Position {
    #[must_use]
    pub fn is_sq_attacked(&self, sq: Square, side: Side) -> bool {
        debug_assert!(sq.0 < 64);

        let bb = Bitboard::from_square(sq);
        let ksq = (self.get_kings() & self.get_side(side)).lsb();

        let bq = self.get_side(side) & (self.get_bishops() | self.get_queens());
        let rq = self.get_side(side) & (self.get_rooks() | self.get_queens());

        // Pawns
        if side == Side::Us
            && rays::pawns::<true>(self.get_pawns() & self.get_side(side)).is_set(sq)
        {
            true
        } else if side == Side::Them
            && rays::pawns::<false>(self.get_pawns() & self.get_side(side)).is_set(sq)
        {
            return true;
        }
        // Knights
        else if (rays::knights(bb) & self.get_knights() & self.get_side(side)).is_occupied() {
            return true;
        }
        // Bishops - Queens
        else if (magic::bishop_moves(sq.0 as i32, self.get_occupied().0) & bq).is_occupied() {
            return true;
        }
        // Rooks - Queens
        else if (magic::rook_moves(sq.0 as i32, self.get_occupied().0) & rq).is_occupied() {
            return true;
        }
        // King
        else if rays::adjacent(ksq).is_set(sq) {
            return true;
        }
        // None
        else {
            return false;
        }
    }

    #[must_use]
    pub fn is_bb_attacked(&self, bb: Bitboard, side: Side) -> bool {
        // Pawns
        if side == Side::Us
            && (rays::pawns::<true>(self.get_pawns() & self.get_us()) & bb).is_occupied()
        {
            return true;
        } else if side == Side::Them
            && (rays::pawns::<false>(self.get_pawns() & self.get_them()) & bb).is_occupied()
        {
            return true;
        }
        // Knights
        else if (rays::knights(bb) & self.get_knights() & self.get_side(side)).is_occupied() {
            return true;
        }
        // King
        else if (rays::adjacent_bb(bb) & self.get_kings() & self.get_side(side)).is_occupied() {
            return true;
        }

        let bq = self.get_side(side) & (self.get_bishops() | self.get_queens());
        let rq = self.get_side(side) & (self.get_rooks() | self.get_queens());

        for sq in bb {
            // Queens
            if (magic::bishop_moves(sq.0 as i32, self.get_occupied().0) & bq).is_occupied()
                || (magic::rook_moves(sq.0 as i32, self.get_occupied().0) & rq).is_occupied()
            {
                return true;
            }
        }

        false
    }

    #[must_use]
    pub fn get_attacked(&self, mask: Bitboard, side: Side) -> Bitboard {
        let mut attacked = Bitboard::empty();

        // Pawns
        if side == Side::Us {
            attacked |= mask & rays::pawns::<true>(self.get_pawns() & self.get_us());
        } else {
            attacked |= mask & rays::pawns::<false>(self.get_pawns() & self.get_them());
        }

        // Knights
        attacked |= mask & rays::knights(self.get_knights() & self.get_side(side));

        // King
        attacked |= mask & rays::adjacent_bb(self.get_kings() & self.get_side(side));

        let bq = self.get_side(side) & (self.get_bishops() | self.get_queens());
        let rq = self.get_side(side) & (self.get_rooks() | self.get_queens());

        for sq in mask & !attacked {
            let bb = Bitboard::from_square(sq);

            // Queens
            if (magic::bishop_moves(sq.0 as i32, self.get_occupied().0) & bq).is_occupied()
                || (magic::rook_moves(sq.0 as i32, self.get_occupied().0) & rq).is_occupied()
            {
                attacked |= bb;
            }
        }

        attacked
    }

    #[must_use]
    pub fn in_check(&self) -> bool {
        let ksq = (self.get_kings() & self.get_us()).lsb();
        self.is_sq_attacked(ksq, Side::Them)
    }

    #[must_use]
    pub fn in_check_them(&self) -> bool {
        let ksq = (self.get_kings() & self.get_them()).lsb();
        self.is_sq_attacked(ksq, Side::Us)
    }

    #[must_use]
    pub fn attacker_paths(&self, sq: Square) -> (Bitboard, Bitboard) {
        let mut attackers = Bitboard::empty();
        let mut paths = Bitboard::empty();

        let bb = Bitboard::from_square(sq);
        let bq = self.get_them() & (self.get_bishops() | self.get_queens());
        let rq = self.get_them() & (self.get_rooks() | self.get_queens());

        // Pawns
        attackers |= rays::pawns::<true>(bb) & self.get_pawns() & self.get_them();

        // Knights
        attackers |= rays::knights(bb) & self.get_knights() & self.get_them();

        // Bishops, Queens
        if bq.is_occupied() {
            let nw = rays::ray_nw(sq, self.get_occupied());
            let ne = rays::ray_ne(sq, self.get_occupied());
            let sw = rays::ray_sw(sq, self.get_occupied());
            let se = rays::ray_se(sq, self.get_occupied());

            if (nw & bq).is_occupied() {
                attackers |= nw & self.get_occupied();
                paths |= nw;
            }

            if (ne & bq).is_occupied() {
                attackers |= ne & self.get_occupied();
                paths |= ne;
            }

            if (sw & bq).is_occupied() {
                attackers |= sw & self.get_occupied();
                paths |= sw;
            }

            if (se & bq).is_occupied() {
                attackers |= se & self.get_occupied();
                paths |= se;
            }
        }

        // Rooks, Queens
        if rq.is_occupied() {
            let n = rays::ray_n(sq, self.get_occupied());
            let e = rays::ray_e(sq, self.get_occupied());
            let s = rays::ray_s(sq, self.get_occupied());
            let w = rays::ray_w(sq, self.get_occupied());

            if (n & rq).is_occupied() {
                attackers |= n & self.get_occupied();
                paths |= n;
            }

            if (e & rq).is_occupied() {
                attackers |= e & self.get_occupied();
                paths |= e;
            }

            if (s & rq).is_occupied() {
                attackers |= s & self.get_occupied();
                paths |= s;
            }

            if (w & rq).is_occupied() {
                attackers |= w & self.get_occupied();
                paths |= w;
            }
        }

        (attackers, paths)
    }
}
