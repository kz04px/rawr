use crate::chess::{bitboard::Bitboard, position::Position};

const PIECE_VALUES: [i32; 6] = [100, 300, 325, 500, 900, 0];
const PASSED_PAWNS: [i32; 8] = [0, 10, 20, 30, 40, 60, 90, 0];
const ROOK_OPEN_FILE: i32 = 25;
#[rustfmt::skip]
const PST: [[i32; 64]; 6] = [
    [
          0,  0,  0,  0,  0,  0,  0,  0,
          5, 10, 10,-20,-20, 10, 10,  5,
          5, -5,-10,  0,  0,-10, -5,  5,
          0,  0,  0, 20, 20,  0,  0,  0,
          5,  5, 10, 25, 25, 10,  5,  5,
         10, 10, 20, 30, 30, 20, 10, 10,
         50, 50, 50, 50, 50, 50, 50, 50,
          0,  0,  0,  0,  0,  0,  0,  0,
    ], [
        -50,-40,-30,-30,-30,-30,-40,-50,
        -40,-20,  0,  5,  5,  0,-20,-40,
        -30,  5, 10, 15, 15, 10,  5,-30,
        -30,  0, 15, 20, 20, 15,  0,-30,
        -30,  5, 15, 20, 20, 15,  5,-30,
        -30,  0, 10, 15, 15, 10,  0,-30,
        -40,-20,  0,  0,  0,  0,-20,-40,
        -50,-40,-30,-30,-30,-30,-40,-50,
    ], [
        -20,-10,-10,-10,-10,-10,-10,-20,
        -10,  5,  0,  0,  0,  0,  5,-10,
        -10, 10, 10, 10, 10, 10, 10,-10,
        -10,  0, 10, 10, 10, 10,  0,-10,
        -10,  5,  5, 10, 10,  5,  5,-10,
        -10,  0,  5, 10, 10,  5,  0,-10,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -20,-10,-10,-10,-10,-10,-10,-20,
    ], [
          0,  0,  0,  5,  5,  0,  0,  0,
         -5,  0,  0,  0,  0,  0,  0, -5,
         -5,  0,  0,  0,  0,  0,  0, -5,
         -5,  0,  0,  0,  0,  0,  0, -5,
         -5,  0,  0,  0,  0,  0,  0, -5,
         -5,  0,  0,  0,  0,  0,  0, -5,
          5, 10, 10, 10, 10, 10, 10,  5,
          0,  0,  0,  0,  0,  0,  0,  0,
    ], [
        -20,-10,-10, -5, -5,-10,-10,-20,
        -10,  0,  5,  0,  0,  0,  0,-10,
        -10,  5,  5,  5,  5,  5,  0,-10,
          0,  0,  5,  5,  5,  5,  0, -5,
         -5,  0,  5,  5,  5,  5,  0, -5,
        -10,  0,  5,  5,  5,  5,  0,-10,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -20,-10,-10, -5, -5,-10,-10,-20,
    ], [
         20, 30, 10,  0,  0, 10, 30, 20,
         20, 20,  0,  0,  0,  0, 20, 20,
        -10,-20,-20,-20,-20,-20,-20,-10,
        -20,-30,-30,-40,-40,-30,-30,-20,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
    ]
];

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
pub fn eval_us(pos: &Position) -> i32 {
    let mut score = 0;
    let pawns_us = pos.get_pawns() & pos.get_us();
    let pawns_them = pos.get_pawns() & pos.get_them();
    let open_files = get_open_files(pos.get_pawns());

    // Passed pawns
    let passed_pawns = get_passed_pawns(pawns_us, pawns_them);
    for sq in passed_pawns {
        let rank = sq.rank();
        score += PASSED_PAWNS[rank as usize];
    }

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
    let mut score = 0;
    score += eval_us(pos);
    score -= eval_us(&Position::from_flipped(pos));
    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::bitboard::Bitboard;

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
}
