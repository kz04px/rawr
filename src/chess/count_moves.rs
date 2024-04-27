use crate::chess::attacks::is_safe;
use crate::chess::bitboard::Bitboard;
use crate::chess::magic;
use crate::chess::position::Position;
use crate::chess::rays;
use crate::chess::side::Side;
use crate::chess::square::Square;
use crate::chess::square::{C1, D1, F1, G1};

#[must_use]
fn line_between(sq1: Square, sq2: Square) -> Bitboard {
    ((Bitboard((1u64 << sq1.0) - 1) ^ Bitboard((1u64 << sq2.0) - 1))
        & !Bitboard::from_square(sq1)
        & !Bitboard::from_square(sq2))
        | Bitboard::from_square(sq2)
}

impl Position {
    #[must_use]
    pub fn count_moves(&self) -> usize {
        let ksq = (self.get_kings() & self.get_us()).lsb();

        let mut ray_ne = Bitboard::empty();
        let mut ray_sw = Bitboard::empty();
        let mut ray_nw = Bitboard::empty();
        let mut ray_se = Bitboard::empty();
        let mut ray_n = Bitboard::empty();
        let mut ray_s = Bitboard::empty();
        let mut ray_e = Bitboard::empty();
        let mut ray_w = Bitboard::empty();

        if (self.get_them() & (self.get_bishops() | self.get_queens())).is_occupied() {
            ray_ne = rays::ray_ne(ksq, self.get_occupied());
            ray_sw = rays::ray_sw(ksq, self.get_occupied());
            ray_nw = rays::ray_nw(ksq, self.get_occupied());
            ray_se = rays::ray_se(ksq, self.get_occupied());
        }

        if (self.get_them() & (self.get_rooks() | self.get_queens())).is_occupied() {
            ray_n = rays::ray_n(ksq, self.get_occupied());
            ray_s = rays::ray_s(ksq, self.get_occupied());
            ray_e = rays::ray_e(ksq, self.get_occupied());
            ray_w = rays::ray_w(ksq, self.get_occupied());
        }

        let bishop_rays = ray_ne | ray_sw | ray_nw | ray_se;
        let rook_rays = ray_n | ray_s | ray_e | ray_w;

        let pawn_attackers = ((self.get_us() & self.get_kings()).north_east()
            | (self.get_us() & self.get_kings()).north_west())
            & self.get_them()
            & self.get_pawns();
        let knight_attackers =
            rays::knights(Bitboard::from_square(ksq)) & self.get_knights() & self.get_them();
        let bishop_attackers =
            bishop_rays & self.get_them() & (self.get_bishops() | self.get_queens());
        let rook_attackers = rook_rays & self.get_them() & (self.get_rooks() | self.get_queens());
        let all_attackers = pawn_attackers | knight_attackers | bishop_attackers | rook_attackers;

        let in_check = all_attackers.is_occupied();

        let allowed =
            // Multiple attackers
            if all_attackers.count() > 1 {
                Bitboard(0x0)
            }
            // Bishop attackers
            else if (ray_ne & bishop_attackers).is_occupied() {
                ray_ne
            } else if (ray_nw & bishop_attackers).is_occupied() {
                ray_nw
            } else if (ray_se & bishop_attackers).is_occupied() {
                ray_se
            } else if (ray_sw & bishop_attackers).is_occupied() {
                ray_sw
            }
            // Rook attackers
            else if (ray_n & rook_attackers).is_occupied() {
                ray_n
            } else if (ray_e & rook_attackers).is_occupied() {
                ray_e
            } else if (ray_s & rook_attackers).is_occupied() {
                ray_s
            } else if (ray_w & rook_attackers).is_occupied() {
                ray_w
            }
            // HMM attackers
            else if all_attackers.is_occupied() {
                debug_assert!(all_attackers.count() == 1);
                all_attackers
            }
            // No attackers
            else {
                !self.get_us()
            };

        let (bpinned, bxrays) = {
            let mut pinned = Bitboard::empty();
            let mut xrays = Bitboard::empty();
            let checkers = self.get_them() & (self.get_bishops() | self.get_queens());

            if (ray_ne & self.get_us()).is_occupied() {
                let sq = (ray_ne & self.get_us()).lsb();
                let xray = rays::ray_ne(sq, self.get_occupied());
                if (xray & checkers).is_occupied() {
                    pinned |= Bitboard::from_square(sq);
                    xrays |= xray | ray_ne;
                }
            }

            if (ray_nw & self.get_us()).is_occupied() {
                let sq = (ray_nw & self.get_us()).lsb();
                let xray = rays::ray_nw(sq, self.get_occupied());
                if (xray & checkers).is_occupied() {
                    pinned |= Bitboard::from_square(sq);
                    xrays |= xray | ray_nw;
                }
            }

            if (ray_se & self.get_us()).is_occupied() {
                let sq = (ray_se & self.get_us()).lsb();
                let xray = rays::ray_se(sq, self.get_occupied());
                if (xray & checkers).is_occupied() {
                    pinned |= Bitboard::from_square(sq);
                    xrays |= xray | ray_se;
                }
            }

            if (ray_sw & self.get_us()).is_occupied() {
                let sq = (ray_sw & self.get_us()).lsb();
                let xray = rays::ray_sw(sq, self.get_occupied());
                if (xray & checkers).is_occupied() {
                    pinned |= Bitboard::from_square(sq);
                    xrays |= xray | ray_sw;
                }
            }

            debug_assert!(pinned.count() <= 4);

            (pinned, xrays | (self.get_kings() & self.get_us()))
        };

        let (vpinned, vxrays) = {
            let mut pinned = Bitboard::empty();
            let mut xrays = Bitboard::empty();

            if (ray_n & self.get_us()).is_occupied() {
                let sq = (ray_n & self.get_us()).lsb();
                let xray = rays::ray_n(sq, self.get_occupied());
                if (xray & self.get_them() & (self.get_rooks() | self.get_queens())).is_occupied() {
                    pinned |= Bitboard::from_square(sq);
                    xrays |= xray | ray_n;
                }
            }

            if (ray_s & self.get_us()).is_occupied() {
                let sq = (ray_s & self.get_us()).lsb();
                let xray = rays::ray_s(sq, self.get_occupied());
                if (xray & self.get_them() & (self.get_rooks() | self.get_queens())).is_occupied() {
                    pinned |= Bitboard::from_square(sq);
                    xrays |= xray | ray_s;
                }
            }

            debug_assert!(pinned.count() <= 2);

            (pinned, xrays)
        };

        let (hpinned, hxrays) = {
            let mut pinned = Bitboard::empty();
            let mut xrays = Bitboard::empty();

            if (ray_e & self.get_us()).is_occupied() {
                let sq = (ray_e & self.get_us()).lsb();
                let xray = rays::ray_e(sq, self.get_occupied());
                if (xray & self.get_them() & (self.get_rooks() | self.get_queens())).is_occupied() {
                    pinned |= Bitboard::from_square(sq);
                    xrays |= xray | ray_e;
                }
            }

            if (ray_w & self.get_us()).is_occupied() {
                let sq = (ray_w & self.get_us()).lsb();
                let xray = rays::ray_w(sq, self.get_occupied());
                if (xray & self.get_them() & (self.get_rooks() | self.get_queens())).is_occupied() {
                    pinned |= Bitboard::from_square(sq);
                    xrays |= xray | ray_w;
                }
            }

            debug_assert!(pinned.count() <= 2);

            (pinned, xrays)
        };

        let rxrays = hxrays | vxrays;
        let rpinned = vpinned | hpinned;
        let pinned = bpinned | rpinned;

        let pawns_promo = self.get_pawns() & self.get_us() & Bitboard(0xFF000000000000);
        let pawns_nonpromo = self.get_pawns() & self.get_us() & Bitboard(0xFFFFFFFFFFFF);

        let mut count = 0;

        // Pawns -- singles
        count +=
            4 * ((pawns_promo & !(hpinned | bpinned)).north() & self.get_empty() & allowed).count();
        count +=
            ((pawns_nonpromo & !(hpinned | bpinned)).north() & self.get_empty() & allowed).count();

        // Pawns -- doubles
        count += ((self.get_pawns() & self.get_us() & !(hpinned | bpinned)).north_north()
            & self.get_empty()
            & self.get_empty().north()
            & Bitboard(0xFF000000)
            & allowed)
            .count();

        // Pawns -- capture NE
        count += 4
            * ((pawns_promo & !rpinned & (!bpinned | bxrays.south_west())).north_east()
                & self.get_them()
                & allowed)
                .count();
        count += ((pawns_nonpromo & !rpinned & (!bpinned | bxrays.south_west())).north_east()
            & self.get_them()
            & allowed)
            .count();

        // Pawns -- capture NW
        count += 4
            * ((pawns_promo & !rpinned & (!bpinned | bxrays.south_east())).north_west()
                & self.get_them()
                & allowed)
                .count();
        count += ((pawns_nonpromo & !rpinned & (!bpinned | bxrays.south_east())).north_west()
            & self.get_them()
            & allowed)
            .count();

        // Pawns -- en passant
        if let Some(ep) = self.ep {
            let rq = self.get_them() & (self.get_rooks() | self.get_queens());

            // North east
            if (self.get_us() & self.get_pawns() & !rpinned & (!bpinned | !bxrays.south_east()))
                .north_east()
                .is_set(ep)
            {
                let blockers = self.get_occupied()
                    ^ Bitboard::from_square(ep)
                    ^ Bitboard::from_square(ep).south()
                    ^ Bitboard::from_square(ep).south_west();

                if (allowed.is_set(ep) || allowed.north().is_set(ep))
                    && ((rays::ray_e(ksq, blockers) & rq).is_empty()
                        && (rays::ray_w(ksq, blockers) & rq).is_empty())
                {
                    count += 1;
                }
            }

            // North west
            if (self.get_us() & self.get_pawns() & !rpinned & (!bpinned | !bxrays.south_west()))
                .north_west()
                .is_set(ep)
            {
                let blockers = self.get_occupied()
                    ^ Bitboard::from_square(ep)
                    ^ Bitboard::from_square(ep).south()
                    ^ Bitboard::from_square(ep).south_east();

                if (allowed.is_set(ep) || allowed.north().is_set(ep))
                    && ((rays::ray_e(ksq, blockers) & rq).is_empty()
                        && (rays::ray_w(ksq, blockers) & rq).is_empty())
                {
                    count += 1;
                }
            }
        }

        // Knights
        for from in self.get_knights() & self.get_us() & !pinned {
            count += (rays::knights(Bitboard::from_square(from)) & allowed).count()
        }

        // Bishops - bishop pinned
        for from in self.get_bishops() & self.get_us() & bpinned {
            let mask = magic::bishop_moves(from.0 as i32, self.get_occupied().0);
            count += (mask & allowed & bxrays).count()
        }
        // Bishops - not pinned
        for from in self.get_bishops() & self.get_us() & !pinned {
            let mask = magic::bishop_moves(from.0 as i32, self.get_occupied().0);
            count += (mask & allowed).count();
        }

        // Rooks - rook pinned
        for from in self.get_rooks() & self.get_us() & rpinned {
            let mask = magic::rook_moves(from.0 as i32, self.get_occupied().0);
            count += (mask & allowed & rxrays).count()
        }
        // Rooks - not pinned
        for from in self.get_rooks() & self.get_us() & !pinned {
            let mask = magic::rook_moves(from.0 as i32, self.get_occupied().0);
            count += (mask & allowed).count();
        }

        // Queens - bishop pinned
        for from in self.get_queens() & self.get_us() & bpinned {
            let mask = magic::bishop_moves(from.0 as i32, self.get_occupied().0);
            count += (mask & allowed & bxrays).count();
        }
        // Queens - rook pinned
        for from in self.get_queens() & self.get_us() & rpinned {
            let mask = magic::rook_moves(from.0 as i32, self.get_occupied().0);
            count += (mask & allowed & rxrays).count();
        }
        // Queens - not pinned
        for from in self.get_queens() & self.get_us() & !pinned {
            let mask = magic::queen_moves(from.0 as i32, self.get_occupied().0);
            count += (mask & allowed).count();
        }

        // King
        for from in self.get_kings() & self.get_us() {
            let kbb = Bitboard::from_square(from);
            for to in Bitboard::from_square(from).adjacent() & !self.get_us() {
                if is_safe(
                    to,
                    self.get_occupied() ^ kbb,
                    self.get_them() & self.get_pawns(),
                    self.get_them() & self.get_knights(),
                    self.get_them() & self.get_bishops(),
                    self.get_them() & self.get_rooks(),
                    self.get_them() & self.get_queens(),
                    self.get_them() & self.get_kings(),
                ) {
                    count += 1;
                }
            }
        }

        let ksc_sq = Square::from_coords(self.castle_files[0], 0);
        let ksc_king_path = line_between(ksq, G1);
        let ksc_rook_path = line_between(ksc_sq, F1);
        let ksc_both_path = ksc_king_path | ksc_rook_path;

        let qsc_sq = Square::from_coords(self.castle_files[1], 0);
        let qsc_king_path = line_between(ksq, C1);
        let qsc_rook_path = line_between(qsc_sq, D1);
        let qsc_both_path = qsc_king_path | qsc_rook_path;

        // King side castling
        if self.us_ksc
            && !in_check
            && !hpinned.is_set(ksc_sq)
            && (self.get_occupied()
                & ksc_both_path
                & !Bitboard::from_square(ksq)
                & !Bitboard::from_square(ksc_sq))
            .is_empty()
            && !self.is_bb_attacked(ksc_king_path, Side::Them)
        {
            count += 1;
        }

        // Queen side castling
        if self.us_qsc
            && !in_check
            && !hpinned.is_set(qsc_sq)
            && (self.get_occupied()
                & qsc_both_path
                & !Bitboard::from_square(ksq)
                & !Bitboard::from_square(qsc_sq))
            .is_empty()
            && !self.is_bb_attacked(qsc_king_path, Side::Them)
        {
            count += 1;
        }

        count as usize
    }
}
