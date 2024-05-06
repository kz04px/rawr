use crate::chess::{bitboard::Bitboard, position::Position, square::Square};

use super::score::Score;

const PIECE_VALUES: [Score; 6] = [
    Score(100, 100),
    Score(300, 300),
    Score(325, 325),
    Score(500, 500),
    Score(900, 900),
    Score(0, 0),
];
const PASSED_PAWNS: [Score; 8] = [
    Score(0, 0),
    Score(10, 10),
    Score(20, 20),
    Score(30, 30),
    Score(40, 40),
    Score(60, 60),
    Score(90, 90),
    Score(0, 0),
];
const ROOK_OPEN_FILE: Score = Score(25, 25);
const KING_PAWN_SHIELD: Score = Score(20, 0);
#[rustfmt::skip]
const PST: [[Score; 64]; 6] = [[
    Score(0, 0),     Score(0, 0),     Score(0, 0),     Score(0, 0),     Score(0, 0),     Score(0, 0),     Score(0, 0),     Score(0, 0),     
    Score(-1, -17),  Score(-7, -17),  Score(-11, -17), Score(-35, -17), Score(-13, -17), Score(5, -17),   Score(3, -17),   Score(-5, -17),  
    Score(1, -11),   Score(1, -11),   Score(-6, -11),  Score(-19, -11), Score(-6, -11),  Score(-7, -11),  Score(-4, -11),  Score(10, -11),  
    Score(1, -7),    Score(14, -7),   Score(8, -7),    Score(4, -7),    Score(5, -7),    Score(4, -7),    Score(10, -7),   Score(7, -7),    
    Score(9, 16),    Score(30, 16),   Score(23, 16),   Score(31, 16),   Score(31, 16),   Score(23, 16),   Score(17, 16),   Score(11, 16),   
    Score(21, 55),   Score(54, 55),   Score(72, 55),   Score(56, 55),   Score(77, 55),   Score(95, 55),   Score(71, 55),   Score(11, 55),   
    Score(118, 82),  Score(121, 82),  Score(173, 82),  Score(168, 82),  Score(107, 82),  Score(82, 82),   Score(-16, 82),  Score(22, 82),   
    Score(0, 0),     Score(0, 0),     Score(0, 0),     Score(0, 0),     Score(0, 0),     Score(0, 0),     Score(0, 0),     Score(0, 0),     
],[
    Score(-99, -99), Score(-30, -99), Score(-66, -94), Score(-64, -88), Score(-29, -88), Score(-19, -94), Score(-61, -99), Score(-81, -99), 
    Score(-56, -81), Score(-31, -62), Score(-28, -49), Score(-1, -43),  Score(-7, -43),  Score(-20, -49), Score(-42, -62), Score(-11, -81), 
    Score(-38, -46), Score(-16, -27), Score(0, -15),   Score(14, -9),   Score(8, -9),    Score(3, -15),   Score(3, -27),   Score(-42, -46), 
    Score(-14, -22), Score(0, -3),    Score(2, 10),    Score(3, 16),    Score(19, 16),   Score(12, 10),   Score(33, -3),   Score(-7, -22),  
    Score(-14, -7),  Score(-4, 12),   Score(25, 25),   Score(33, 31),   Score(10, 31),   Score(33, 25),   Score(14, 12),   Score(43, -7),   
    Score(-22, -2),  Score(18, 17),   Score(60, 30),   Score(64, 36),   Score(124, 36),  Score(143, 30),  Score(55, 17),   Score(6, -2),    
    Score(-34, -7),  Score(24, 12),   Score(54, 25),   Score(74, 31),   Score(60, 31),   Score(122, 25),  Score(2, 12),    Score(29, -7),   
    Score(-60, -21), Score(0, -3),    Score(0, 10),    Score(0, 16),    Score(0, 16),    Score(0, 10),    Score(0, -3),    Score(0, -21),   
],[
    Score(-7, -27),  Score(12, -21),  Score(-8, -17),  Score(-37, -15), Score(-31, -15), Score(-8, -17),  Score(-45, -21), Score(-67, -27), 
    Score(15, -10),  Score(5, -4),    Score(13, 0),    Score(-10, 2),   Score(1, 2),     Score(2, 0),     Score(0, -4),    Score(15, -10),  
    Score(5, 2),     Score(12, 8),    Score(14, 12),   Score(13, 14),   Score(10, 14),   Score(-1, 12),   Score(3, 8),     Score(4, 2),     
    Score(1, 11),    Score(5, 17),    Score(23, 21),   Score(32, 23),   Score(21, 23),   Score(8, 21),    Score(17, 17),   Score(4, 11),    
    Score(-1, 14),   Score(16, 20),   Score(29, 24),   Score(27, 26),   Score(37, 26),   Score(27, 24),   Score(17, 20),   Score(4, 14),    
    Score(7, 13),    Score(27, 19),   Score(20, 23),   Score(56, 25),   Score(91, 25),   Score(108, 23),  Score(53, 19),   Score(44, 13),   
    Score(-24, 8),   Score(-23, 14),  Score(30, 18),   Score(58, 20),   Score(65, 20),   Score(61, 18),   Score(69, 14),   Score(11, 8),    
    Score(0, -2),    Score(0, 4),     Score(0, 8),     Score(0, 10),    Score(0, 10),    Score(0, 8),     Score(0, 4),     Score(0, -2),    
],[
    Score(-2, -32),  Score(-1, -31),  Score(3, -30),   Score(1, -29),   Score(2, -29),   Score(1, -30),   Score(4, -31),   Score(-8, -32),  
    Score(-26, -27), Score(-6, -25),  Score(2, -24),   Score(-2, -24),  Score(2, -24),   Score(-10, -24), Score(-1, -25),  Score(-29, -27), 
    Score(-16, -15), Score(0, -13),   Score(3, -12),   Score(-3, -12),  Score(8, -12),   Score(-1, -12),  Score(12, -13),  Score(3, -15),   
    Score(-9, 1),    Score(-5, 2),    Score(8, 3),     Score(14, 4),    Score(18, 4),    Score(-17, 3),   Score(13, 2),    Score(-13, 1),   
    Score(19, 15),   Score(33, 17),   Score(46, 18),   Score(57, 18),   Score(53, 18),   Score(39, 18),   Score(53, 17),   Score(16, 15),   
    Score(24, 25),   Score(83, 27),   Score(54, 28),   Score(75, 28),   Score(134, 28),  Score(144, 28),  Score(85, 27),   Score(75, 25),   
    Score(46, 27),   Score(33, 28),   Score(64, 29),   Score(62, 30),   Score(91, 30),   Score(89, 29),   Score(70, 28),   Score(104, 27),  
    Score(84, 16),   Score(0, 17),    Score(0, 18),    Score(37, 19),   Score(124, 19),  Score(0, 18),    Score(0, 17),    Score(153, 16),  
],[
    Score(1, -61),   Score(-10, -55), Score(-11, -52), Score(3, -50),   Score(-15, -50), Score(-51, -52), Score(-83, -55), Score(-13, -61), 
    Score(-7, -31),  Score(3, -26),   Score(2, -22),   Score(5, -21),   Score(-1, -21),  Score(-10, -22), Score(-7, -26),  Score(-2, -31),  
    Score(-11, -8),  Score(0, -3),    Score(12, 1),    Score(2, 3),     Score(8, 3),     Score(11, 1),    Score(7, -3),    Score(-6, -8),   
    Score(-9, 9),    Score(5, 14),    Score(7, 17),    Score(9, 19),    Score(18, 19),   Score(17, 17),   Score(26, 14),   Score(4, 9),     
    Score(-6, 19),   Score(0, 24),    Score(15, 28),   Score(25, 30),   Score(32, 30),   Score(9, 28),    Score(26, 24),   Score(12, 19),   
    Score(-16, 23),  Score(10, 28),   Score(13, 32),   Score(25, 34),   Score(37, 34),   Score(30, 32),   Score(15, 28),   Score(26, 23),   
    Score(1, 21),    Score(11, 26),   Score(35, 30),   Score(0, 31),    Score(16, 31),   Score(55, 30),   Score(39, 26),   Score(57, 21),   
    Score(-13, 12),  Score(6, 17),    Score(-42, 21),  Score(0, 23),    Score(29, 23),   Score(0, 21),    Score(0, 17),    Score(102, 12),  
],[
    Score(0, -34),   Score(0, -30),   Score(0, -28),   Score(-9, -27),  Score(0, -27),   Score(-9, -28),  Score(25, -30),  Score(0, -34),   
    Score(-9, -17),  Score(-9, -13),  Score(-9, -11),  Score(-9, -10),  Score(-9, -10),  Score(-9, -11),  Score(-9, -13),  Score(-9, -17),  
    Score(-9, -2),   Score(-9, 2),    Score(-9, 4),    Score(-9, 5),    Score(-9, 5),    Score(-9, 4),    Score(-9, 2),    Score(-9, -2),   
    Score(-9, 11),   Score(-9, 15),   Score(-9, 17),   Score(-9, 18),   Score(-9, 18),   Score(-9, 17),   Score(-9, 15),   Score(-9, 11),   
    Score(-9, 22),   Score(-9, 26),   Score(-9, 28),   Score(-9, 29),   Score(-9, 29),   Score(-9, 28),   Score(-9, 26),   Score(-9, 22),   
    Score(-9, 31),   Score(-9, 34),   Score(-9, 37),   Score(-9, 38),   Score(-9, 38),   Score(-9, 37),   Score(-9, 34),   Score(-9, 31),   
    Score(-9, 38),   Score(-9, 41),   Score(-9, 44),   Score(-9, 45),   Score(-9, 45),   Score(-9, 44),   Score(-9, 41),   Score(-9, 38),   
    Score(-9, 42),   Score(-9, 46),   Score(-9, 48),   Score(-9, 50),   Score(-9, 50),   Score(-9, 48),   Score(-9, 46),   Score(-9, 42),   
]];

#[must_use]
fn get_passed_pawns(us: Bitboard, them: Bitboard) -> Bitboard {
    let mut mask = them.south() | them.south_east() | them.south_west();
    mask |= mask.south();
    mask |= mask.south();
    mask |= mask.south();
    mask |= mask.south();
    us & !mask
}

#[must_use]
fn get_open_files(pawns: Bitboard) -> Bitboard {
    !Bitboard(
        pawns.0 << 56
            | pawns.0 << 48
            | pawns.0 << 40
            | pawns.0 << 32
            | pawns.0 << 24
            | pawns.0 << 16
            | pawns.0 << 8
            | pawns.0
            | pawns.0 >> 8
            | pawns.0 >> 16
            | pawns.0 >> 24
            | pawns.0 >> 32
            | pawns.0 >> 40
            | pawns.0 >> 48
            | pawns.0 >> 56,
    )
}

#[must_use]
fn get_king_shield(ksq: Square) -> Bitboard {
    let bb = Bitboard::from_square(ksq);
    bb.north()
        | bb.north_east()
        | bb.north_west()
        | bb.north().north()
        | bb.north().north_east()
        | bb.north().north_west()
}

#[must_use]
fn get_phase(pos: &Position) -> i32 {
    let phase = 24
        - pos.get_knights().count()
        - pos.get_bishops().count()
        - pos.get_rooks().count() * 2
        - pos.get_queens().count() * 4;

    (phase * 256 + 12) / 24
}

#[must_use]
fn taper(score: &Score, phase: i32) -> i32 {
    ((score.mg() * (256 - phase)) + (score.eg() * phase)) / 256
}

#[must_use]
pub fn eval_us(pos: &Position) -> Score {
    let mut score = Score::default();
    let ksq = (pos.get_kings() & pos.get_us()).lsb();
    let pawns_us = pos.get_pawns() & pos.get_us();
    let pawns_them = pos.get_pawns() & pos.get_them();
    let open_files = get_open_files(pos.get_pawns());

    // Passed pawns
    let passed_pawns = get_passed_pawns(pawns_us, pawns_them);
    for sq in passed_pawns {
        let rank = sq.rank();
        score += PASSED_PAWNS[rank as usize];
    }

    // King pawn shield
    score += KING_PAWN_SHIELD * (get_king_shield(ksq) & pawns_us).count();

    // Rooks on open files
    score += ROOK_OPEN_FILE * (open_files & pos.get_us() & pos.get_rooks()).count();

    for i in 0..6 {
        // Material
        score += PIECE_VALUES[i] * (pos.get_piece(i) & pos.get_us()).count();

        // PST
        for sq in pos.get_piece(i) & pos.get_us() {
            score += PST[i][sq.0 as usize];
        }
    }

    score
}

#[must_use]
pub fn eval(pos: &Position) -> i32 {
    let score = eval_us(pos) - eval_us(&Position::from_flipped(pos));
    let phase = get_phase(&pos);
    let tapered = taper(&score, phase);
    tapered
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::{bitboard::Bitboard, square::*};

    #[test]
    fn test_passers() {
        assert_eq!(
            get_passed_pawns(Bitboard::empty(), Bitboard::empty()),
            Bitboard::empty()
        );
        assert_eq!(
            get_passed_pawns(Bitboard(0xff00), Bitboard::empty()),
            Bitboard(0xff00)
        );
        assert_eq!(
            get_passed_pawns(Bitboard(0xff000000000000), Bitboard::empty()),
            Bitboard(0xff000000000000)
        );
        assert_eq!(
            get_passed_pawns(Bitboard(0xff00), Bitboard(0xff0000000000)),
            Bitboard::empty()
        );
        assert_eq!(
            get_passed_pawns(Bitboard(0xff000000000000), Bitboard(0xff0000000000)),
            Bitboard(0xff000000000000)
        );
        assert_eq!(
            get_passed_pawns(Bitboard(0x10000000), Bitboard(0xef100000)),
            Bitboard(0x10000000)
        );
        assert_eq!(
            get_passed_pawns(Bitboard(0x8100), Bitboard(0x81000000000000)),
            Bitboard::empty()
        );
        assert_eq!(
            get_passed_pawns(Bitboard(0x8100), Bitboard(0x42000000000000)),
            Bitboard::empty()
        );
        assert_eq!(
            get_passed_pawns(Bitboard(0x8100), Bitboard(0x3c000000000000)),
            Bitboard(0x8100)
        );
        assert_eq!(
            get_passed_pawns(Bitboard(0xaa000000000000), Bitboard(0x55000000000000)),
            Bitboard(0xaa000000000000)
        );
        assert_eq!(
            get_passed_pawns(Bitboard(0x55000000000000), Bitboard(0xaa000000000000)),
            Bitboard(0x55000000000000)
        );
    }

    #[test]
    fn test_open_files() {
        assert_eq!(get_open_files(Bitboard::empty()), Bitboard::full());
        assert_eq!(get_open_files(Bitboard::full()), Bitboard::empty());
        assert_eq!(
            get_open_files(Bitboard(0x10000000)),
            Bitboard(0xefefefefefefefef)
        );
        assert_eq!(
            get_open_files(Bitboard(0x90000010000201)),
            Bitboard(0x6c6c6c6c6c6c6c6c)
        );
    }

    #[test]
    fn test_king_shield() {
        assert_eq!(
            get_king_shield(Square::from_index(SquareIdx::E4)),
            Bitboard(0x383800000000)
        );
        assert_eq!(
            get_king_shield(Square::from_index(SquareIdx::A1)),
            Bitboard(0x30300)
        );
        assert_eq!(
            get_king_shield(Square::from_index(SquareIdx::H1)),
            Bitboard(0xc0c000)
        );
    }
}
