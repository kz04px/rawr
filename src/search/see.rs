use crate::chess::bitboard::Bitboard;
use crate::chess::magic;
use crate::chess::mv::Mv;
use crate::chess::piece::Piece;
use crate::chess::position::Position;
use crate::chess::rays;
use crate::chess::square::Square;

#[must_use]
fn get_smallest_attacker(
    sq: Square,
    colours: &[Bitboard; 2],
    pieces: &[Bitboard; 6],
) -> Option<(Piece, Square)> {
    let bb = Bitboard::from_square(sq);
    let blockers = colours[0] | colours[1];

    // Pawns
    let attackers_pawn = (bb.south_east() | bb.south_west()) & pieces[0] & colours[0];
    if attackers_pawn.is_occupied() {
        return Some((Piece::Pawn, attackers_pawn.lsb()));
    }

    // Knights
    let attackers_knight = rays::knights(bb) & pieces[1] & colours[0];
    if attackers_knight.is_occupied() {
        return Some((Piece::Knight, attackers_knight.lsb()));
    }

    // Bishops
    let attackers_bishop = magic::bishop_moves(sq.0 as i32, blockers.0) & pieces[2] & colours[0];
    if attackers_bishop.is_occupied() {
        return Some((Piece::Bishop, attackers_bishop.lsb()));
    }

    // Rooks
    let attackers_rook = magic::rook_moves(sq.0 as i32, blockers.0) & pieces[3] & colours[0];
    if attackers_rook.is_occupied() {
        return Some((Piece::Rook, attackers_rook.lsb()));
    }

    // Queens
    let attackers_queen = magic::queen_moves(sq.0 as i32, blockers.0) & pieces[4] & colours[0];
    if attackers_queen.is_occupied() {
        return Some((Piece::Queen, attackers_queen.lsb()));
    }

    // Kings
    let attackers_king = magic::king_moves(sq.0 as i32) & pieces[5] & colours[0];
    if attackers_king.is_occupied() {
        return Some((Piece::King, attackers_king.lsb()));
    }

    // None
    None
}

#[must_use]
fn see_impl(
    sq: Square,
    captured: i32,
    piece_values: &[i32; 6],
    colours: &mut [Bitboard; 2],
    pieces: &mut [Bitboard; 6],
) -> i32 {
    let (attacker, from) = if let Some(asd) = get_smallest_attacker(sq, &colours, &pieces) {
        asd
    } else {
        return 0;
    };

    // make
    colours[0] ^= Bitboard::from_square(from);
    pieces[attacker as usize] ^= Bitboard::from_square(from);

    let score = captured
        - see_impl(
            sq.flip(),
            piece_values[attacker as usize],
            piece_values,
            &mut [colours[1].flip(), colours[0].flip()],
            &mut [
                pieces[0].flip(),
                pieces[1].flip(),
                pieces[2].flip(),
                pieces[3].flip(),
                pieces[4].flip(),
                pieces[5].flip(),
            ],
        );

    // undo
    colours[0] ^= Bitboard::from_square(from);
    pieces[attacker as usize] ^= Bitboard::from_square(from);

    std::cmp::max(score, 0)
}

#[must_use]
pub fn see(pos: &Position, mv: &Mv, piece_values: &[i32; 6]) -> i32 {
    let mut colours = pos.colours;
    let mut pieces = pos.pieces;

    let value = if pos.get_us().is_set(mv.to) {
        return 0;
    } else if let Some(captured) = pos.get_piece_on(mv.to) {
        piece_values[captured as usize]
    } else if pos.ep.is_some()
        && pos.ep.unwrap() == mv.to
        && pos.get_piece_on(mv.from) == Some(Piece::Pawn)
    {
        let bb = Bitboard::from_square(pos.ep.unwrap()).south();
        colours[1] ^= bb;
        pieces[0] ^= bb;
        piece_values[0]
    } else {
        0
    };

    let attacker = pos.get_piece_on(mv.from).unwrap();

    colours[0] ^= Bitboard::from_square(mv.from);
    pieces[attacker as usize] ^= Bitboard::from_square(mv.from);

    let gg = see_impl(
        mv.to.flip(),
        piece_values[attacker as usize],
        piece_values,
        &mut [colours[1].flip(), colours[0].flip()],
        &mut [
            pieces[0].flip(),
            pieces[1].flip(),
            pieces[2].flip(),
            pieces[3].flip(),
            pieces[4].flip(),
            pieces[5].flip(),
        ],
    );

    value - gg
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::piece::Piece;
    use crate::chess::square::Square;
    use crate::chess::square::SquareIdx;

    const VALUES: [i32; 6] = [100, 300, 325, 500, 900, 0];

    #[test]
    fn attacker() {
        let tests = [
            ("3k4/8/8/8/8/8/8/4K3 w - - 0 1", None),
            (
                "3k4/8/8/8/8/5P2/8/4K3 w - - 0 1",
                Some((Piece::Pawn, Square::from_index(SquareIdx::F3))),
            ),
            (
                "3k4/8/8/8/8/8/5N2/4K3 w - - 0 1",
                Some((Piece::Knight, Square::from_index(SquareIdx::F2))),
            ),
            (
                "3k4/8/8/8/8/8/6B1/4K3 w - - 0 1",
                Some((Piece::Bishop, Square::from_index(SquareIdx::G2))),
            ),
            (
                "3k4/8/8/8/1R6/8/8/4K3 w - - 0 1",
                Some((Piece::Rook, Square::from_index(SquareIdx::B4))),
            ),
            (
                "3k4/8/8/8/8/8/2Q5/4K3 w - - 0 1",
                Some((Piece::Queen, Square::from_index(SquareIdx::C2))),
            ),
            (
                "3k4/8/8/8/8/8/4Q3/4K3 w - - 0 1",
                Some((Piece::Queen, Square::from_index(SquareIdx::E2))),
            ),
            (
                "3k4/8/8/8/8/4K3/8/8 w - - 0 1",
                Some((Piece::King, Square::from_index(SquareIdx::E3))),
            ),
            (
                "3k4/8/8/5Q2/2R2K2/3B1P2/4QN2/8 w - - 0 1",
                Some((Piece::Pawn, Square::from_index(SquareIdx::F3))),
            ),
            (
                "3k4/8/8/5Q2/2R2K2/3B4/4QN2/8 w - - 0 1",
                Some((Piece::Knight, Square::from_index(SquareIdx::F2))),
            ),
            (
                "3k4/8/8/5Q2/2R2K2/3B4/4Q3/8 w - - 0 1",
                Some((Piece::Bishop, Square::from_index(SquareIdx::D3))),
            ),
        ];

        for (fen, expected) in tests {
            println!("fen: {}", fen);
            let pos = Position::from_fen(fen);
            let sq = Square::from_index(SquareIdx::E4);
            let mut colours = pos.colours;
            let mut pieces = pos.pieces;
            assert_eq!(
                expected,
                get_smallest_attacker(sq, &mut colours, &mut pieces)
            );
        }
    }

    #[test]
    fn startpos() {
        let pos = Position::startpos();
        let moves = pos.legal_moves();
        for mv in moves {
            assert_eq!(see(&pos, &mv, &VALUES), 0);
        }
    }

    #[test]
    fn basic_noncaptures() {
        let fens = [
            ("3k4/8/8/3p4/4P3/8/8/4K3 w - - 0 1", 0),
            ("3k4/8/8/3n4/4P3/8/8/4K3 w - - 0 1", 0),
            ("3k4/8/8/3q4/4P3/8/8/4K3 w - - 0 1", -100),
            ("3k4/8/8/3p4/4R3/8/8/4K3 w - - 0 1", 0),
            ("3k4/8/8/3n4/4R3/8/8/4K3 w - - 0 1", 0),
            ("3k4/8/8/3q4/4R3/8/8/4K3 w - - 0 1", -500),
        ];

        let mv = Mv {
            from: Square::from_index(SquareIdx::E4),
            to: Square::from_index(SquareIdx::E5),
            promo: Piece::None,
        };

        for (fen, value) in fens {
            println!("fen: {}", fen);
            let pos = Position::from_fen(fen);
            assert_eq!(see(&pos, &mv, &VALUES), value);
        }
    }

    #[test]
    fn basic_captures() {
        let fens = [
            ("4k3/8/8/3p4/4P3/8/8/4K3 w - - 0 1", 100),
            ("4k3/8/8/3n4/4P3/8/8/4K3 w - - 0 1", 300),
            ("4k3/8/8/3q4/4P3/8/8/4K3 w - - 0 1", 900),
            ("4k3/8/8/3p4/4B3/8/8/4K3 w - - 0 1", 100),
            ("4k3/8/8/3n4/4B3/8/8/4K3 w - - 0 1", 300),
            ("4k3/8/8/3q4/4B3/8/8/4K3 w - - 0 1", 900),
        ];

        let mv = Mv {
            from: Square::from_index(SquareIdx::E4),
            to: Square::from_index(SquareIdx::D5),
            promo: Piece::None,
        };

        for (fen, value) in fens {
            println!("fen: {}", fen);
            let pos = Position::from_fen(fen);
            assert_eq!(see(&pos, &mv, &VALUES), value);
        }
    }

    #[test]
    fn advanced_captures() {
        let fens = [
            ("4k3/8/4b3/3p4/2P1P3/8/8/4K3 w - - 0 1", 100),
            ("4k3/8/2p1b3/3p4/2P1P3/8/8/4K3 w - - 0 1", 0),
            ("4k3/8/2p1b3/3p4/2B1P3/8/8/4K3 w - - 0 1", 0),
            ("4k3/8/2p5/3p4/4B3/8/8/4K3 w - - 0 1", -225),
            ("4k3/8/2q1p3/3p4/4P3/8/3R4/3RK3 w - - 0 1", 100),
            ("4k3/8/2q1p3/3p4/4P3/8/3R4/4K3 w - - 0 1", 0),
        ];

        let mv = Mv {
            from: Square::from_index(SquareIdx::E4),
            to: Square::from_index(SquareIdx::D5),
            promo: Piece::None,
        };

        for (fen, value) in fens {
            println!("fen: {}", fen);
            let pos = Position::from_fen(fen);
            assert_eq!(see(&pos, &mv, &VALUES), value);
        }
    }

    #[test]
    fn enpassant() {
        let fens = [
            ("4k3/8/8/3pP3/8/8/8/4K3 w - d6 0 2", 100),
            ("4k3/4p3/8/3pP3/8/8/8/4K3 w - d6 0 2", 0),
            ("4k3/4p3/8/3pP3/1B6/8/8/4K3 w - d6 0 2", 100),
            ("4k3/2p1p3/8/3pP3/1B6/8/8/4K3 w - d6 0 2", 0),
            ("4k3/8/8/3pP3/8/3r4/8/4K3 w - d6 0 2", 0),
        ];

        let mv = Mv {
            from: Square::from_index(SquareIdx::E5),
            to: Square::from_index(SquareIdx::D6),
            promo: Piece::None,
        };

        for (fen, value) in fens {
            println!("fen: {}", fen);
            let pos = Position::from_fen(fen);
            assert_eq!(see(&pos, &mv, &VALUES), value);
        }
    }

    #[test]
    fn castling() {
        let fens = [
            (
                "4k3/8/8/8/8/8/8/4K2R w K - 0 1",
                SquareIdx::E1,
                SquareIdx::H1,
                0,
            ),
            (
                "4k3/8/8/8/8/8/8/R3K3 w Q - 0 1",
                SquareIdx::E1,
                SquareIdx::A1,
                0,
            ),
            (
                "4k3/8/8/8/8/8/8/3K2R1 w G - 0 1",
                SquareIdx::D1,
                SquareIdx::G1,
                0,
            ),
            (
                "4k3/8/8/8/8/8/8/2RK4 w C - 0 1",
                SquareIdx::D1,
                SquareIdx::C1,
                0,
            ),
        ];

        for (fen, from, to, value) in fens {
            println!("fen: {}", fen);
            let pos = Position::from_fen(fen);
            let mv = Mv {
                from: Square::from_index(from),
                to: Square::from_index(to),
                promo: Piece::None,
            };
            assert_eq!(see(&pos, &mv, &VALUES), value);
        }
    }

    #[test]
    fn many_captures() {
        let mv = Mv {
            from: Square::from_index(SquareIdx::E4),
            to: Square::from_index(SquareIdx::D5),
            promo: Piece::None,
        };

        let pos =
            Position::from_fen("b2rk1b1/1b1r1b2/2b1b3/qq4qQ/2B1B3/1B1Q1B2/B2R2B1/3RK2B w - - 0 1");
        let asd = see(&pos, &mv, &VALUES);
        println!("{}", asd);
    }
}
