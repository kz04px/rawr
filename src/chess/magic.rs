use crate::chess::{bitboard::Bitboard, square::Square};

const BISHOP_STUFF: [(u64, i32); 64] = [
    (0x007fbfbfbfbfbfff, 5378),
    (0x0000a060401007fc, 4093),
    (0x0001004008020000, 4314),
    (0x0000806004000000, 6587),
    (0x0000100400000000, 6491),
    (0x000021c100b20000, 6330),
    (0x0000040041008000, 5609),
    (0x00000fb0203fff80, 22236),
    (0x0000040100401004, 6106),
    (0x0000020080200802, 5625),
    (0x0000004010202000, 16785),
    (0x0000008060040000, 16817),
    (0x0000004402000000, 6842),
    (0x0000000801008000, 7003),
    (0x000007efe0bfff80, 4197),
    (0x0000000820820020, 7356),
    (0x0000400080808080, 4602),
    (0x00021f0100400808, 4538),
    (0x00018000c06f3fff, 29531),
    (0x0000258200801000, 45393),
    (0x0000240080840000, 12420),
    (0x000018000c03fff8, 15763),
    (0x00000a5840208020, 5050),
    (0x0000020008208020, 4346),
    (0x0000804000810100, 6074),
    (0x0001011900802008, 7866),
    (0x0000804000810100, 32139),
    (0x000100403c0403ff, 57673),
    (0x00078402a8802000, 55365),
    (0x0000101000804400, 15818),
    (0x0000080800104100, 5562),
    (0x00004004c0082008, 6390),
    (0x0001010120008020, 7930),
    (0x000080809a004010, 13329),
    (0x0007fefe08810010, 7170),
    (0x0003ff0f833fc080, 27267),
    (0x007fe08019003042, 53787),
    (0x003fffefea003000, 5097),
    (0x0000101010002080, 6643),
    (0x0000802005080804, 6138),
    (0x0000808080a80040, 7418),
    (0x0000104100200040, 7898),
    (0x0003ffdf7f833fc0, 42012),
    (0x0000008840450020, 57350),
    (0x00007ffc80180030, 22813),
    (0x007fffdd80140028, 56693),
    (0x00020080200a0004, 5818),
    (0x0000101010100020, 7098),
    (0x0007ffdfc1805000, 4451),
    (0x0003ffefe0c02200, 4709),
    (0x0000000820806000, 4794),
    (0x0000000008403000, 13364),
    (0x0000000100202000, 4570),
    (0x0000004040802000, 4282),
    (0x0004010040100400, 14964),
    (0x00006020601803f4, 4026),
    (0x0003ffdfdfc28048, 4826),
    (0x0000000820820020, 7354),
    (0x0000000008208060, 4848),
    (0x0000000000808020, 15946),
    (0x0000000001002020, 14932),
    (0x0000000401002008, 16588),
    (0x0000004040404040, 6905),
    (0x007fff9fdf7ff813, 16076),
];

const ROOK_STUFF: [(u64, i32); 64] = [
    (0x00280077ffebfffe, 26304),
    (0x2004010201097fff, 35520),
    (0x0010020010053fff, 38592),
    (0x0040040008004002, 8026),
    (0x7fd00441ffffd003, 22196),
    (0x4020008887dffffe, 80870),
    (0x004000888847ffff, 76747),
    (0x006800fbff75fffd, 30400),
    (0x000028010113ffff, 11115),
    (0x0020040201fcffff, 18205),
    (0x007fe80042ffffe8, 53577),
    (0x00001800217fffe8, 62724),
    (0x00001800073fffe8, 34282),
    (0x00001800e05fffe8, 29196),
    (0x00001800602fffe8, 23806),
    (0x000030002fffffa0, 49481),
    (0x00300018010bffff, 2410),
    (0x0003000c0085fffb, 36498),
    (0x0004000802010008, 24478),
    (0x0004002020020004, 10074),
    (0x0001002002002001, 79315),
    (0x0001001000801040, 51779),
    (0x0000004040008001, 13586),
    (0x0000006800cdfff4, 19323),
    (0x0040200010080010, 70612),
    (0x0000080010040010, 83652),
    (0x0004010008020008, 63110),
    (0x0000040020200200, 34496),
    (0x0002008010100100, 84966),
    (0x0000008020010020, 54341),
    (0x0000008020200040, 60421),
    (0x0000820020004020, 86402),
    (0x00fffd1800300030, 50245),
    (0x007fff7fbfd40020, 76622),
    (0x003fffbd00180018, 84676),
    (0x001fffde80180018, 78757),
    (0x000fffe0bfe80018, 37346),
    (0x0001000080202001, 370),
    (0x0003fffbff980180, 42182),
    (0x0001fffdff9000e0, 45385),
    (0x00fffefeebffd800, 61659),
    (0x007ffff7ffc01400, 12790),
    (0x003fffbfe4ffe800, 16762),
    (0x001ffff01fc03000, 0),
    (0x000fffe7f8bfe800, 38380),
    (0x0007ffdfdf3ff808, 11098),
    (0x0003fff85fffa804, 21803),
    (0x0001fffd75ffa802, 39189),
    (0x00ffffd7ffebffd8, 58628),
    (0x007fff75ff7fbfd8, 44116),
    (0x003fff863fbf7fd8, 78357),
    (0x001fffbfdfd7ffd8, 44481),
    (0x000ffff810280028, 64134),
    (0x0007ffd7f7feffd8, 41759),
    (0x0003fffc0c480048, 1394),
    (0x0001ffffafd7ffd8, 40910),
    (0x00ffffe4ffdfa3ba, 66516),
    (0x007fffef7ff3d3da, 3897),
    (0x003fffbfdfeff7fa, 3930),
    (0x001fffeff7fbfc22, 72934),
    (0x0000020408001001, 72662),
    (0x0007fffeffff77fd, 56325),
    (0x0003ffffbf7dfeec, 66501),
    (0x0001ffff9dffa333, 14826),
];

#[must_use]
const fn north(bb: u64) -> u64 {
    bb << 8
}

#[must_use]
const fn east(bb: u64) -> u64 {
    (bb << 1) & 0xfefefefefefefefe
}

#[must_use]
const fn south(bb: u64) -> u64 {
    bb >> 8
}

#[must_use]
const fn west(bb: u64) -> u64 {
    (bb >> 1) & 0x7f7f7f7f7f7f7f7f
}

#[must_use]
const fn calculate_knight_masks() -> [u64; 64] {
    let mut result = [0u64; 64];

    let mut i = 0;
    while i < 64 {
        let bb = 1u64 << i;

        result[i] |= north(north(east(bb)));
        result[i] |= north(north(west(bb)));
        result[i] |= east(east(north(bb)));
        result[i] |= east(east(south(bb)));
        result[i] |= west(west(north(bb)));
        result[i] |= west(west(south(bb)));
        result[i] |= south(south(east(bb)));
        result[i] |= south(south(west(bb)));

        i += 1
    }

    result
}

#[must_use]
const fn calculate_bishop_masks() -> [u64; 64] {
    let mut result = [0u64; 64];

    let mut i = 0;
    while i < 64 {
        let bb = 1u64 << i;

        // Up 1 Right 1
        let mut nbb = north(east(bb));
        while north(east(nbb)) != 0u64 {
            result[i] |= nbb;
            nbb = north(east(nbb));
        }

        // Up 1 Left 1
        let mut nbb = north(west(bb));
        while north(west(nbb)) != 0u64 {
            result[i] |= nbb;
            nbb = north(west(nbb));
        }

        // Down 1 Right 1
        let mut nbb: u64 = south(east(bb));
        while south(east(nbb)) != 0u64 {
            result[i] |= nbb;
            nbb = south(east(nbb));
        }

        // Down 1 Left 1
        let mut nbb: u64 = south(west(bb));
        while south(west(nbb)) != 0u64 {
            result[i] |= nbb;
            nbb = south(west(nbb));
        }

        i += 1;
    }

    result
}

#[must_use]
const fn calculate_rook_masks() -> [u64; 64] {
    let mut result = [0u64; 64];

    let mut i = 0;
    while i < 64 {
        let bb = 1u64 << i;

        // Right
        let mut nbb = east(bb);
        while east(nbb) != 0u64 {
            result[i] |= nbb;
            nbb = east(nbb);
        }

        // Left
        let mut nbb = west(bb);
        while west(nbb) != 0u64 {
            result[i] |= nbb;
            nbb = west(nbb);
        }

        // Up
        let mut nbb = north(bb);
        while north(nbb) != 0u64 {
            result[i] |= nbb;
            nbb = north(nbb);
        }

        // Down
        let mut nbb = south(bb);
        while south(nbb) != 0u64 {
            result[i] |= nbb;
            nbb = south(nbb);
        }

        i += 1;
    }

    result
}

#[must_use]
const fn calculate_king_masks() -> [u64; 64] {
    let mut result = [0u64; 64];

    let mut i = 0;
    while i < 64 {
        let bb = 1u64 << i;

        result[i] = north(bb)
            | north(west(bb))
            | north(east(bb))
            | west(bb)
            | east(bb)
            | south(bb)
            | south(west(bb))
            | south(east(bb));

        i += 1;
    }

    result
}

const KNIGHT_MASKS: [u64; 64] = calculate_knight_masks();
const BISHOP_MASKS: [u64; 64] = calculate_bishop_masks();
const ROOK_MASKS: [u64; 64] = calculate_rook_masks();
const KING_MASKS: [u64; 64] = calculate_king_masks();
include!(concat!(env!("OUT_DIR"), "/magic_constants.rs"));

#[must_use]
const fn bishop_index(sq: i32, blockers: u64) -> usize {
    let relevant_blockers = blockers & BISHOP_MASKS[sq as usize];
    let bishop_magic = BISHOP_STUFF[sq as usize].0;
    let bishop_offset = BISHOP_STUFF[sq as usize].1;
    bishop_offset as usize + (((relevant_blockers).wrapping_mul(bishop_magic)) >> 55) as usize
}

#[must_use]
const fn rook_index(sq: i32, blockers: u64) -> usize {
    let relevant_blockers = blockers & ROOK_MASKS[sq as usize];
    let rook_magic = ROOK_STUFF[sq as usize].0;
    let rook_offset = ROOK_STUFF[sq as usize].1;
    rook_offset as usize + (((relevant_blockers).wrapping_mul(rook_magic)) >> 52) as usize
}

#[must_use]
pub fn knight_moves(sq: Square) -> Bitboard {
    Bitboard(KNIGHT_MASKS[sq.0 as usize])
}

#[must_use]
pub fn bishop_moves(sq: i32, blockers: u64) -> Bitboard {
    let idx = bishop_index(sq, blockers);
    Bitboard(MAGIC_MOVES[idx])
}

#[must_use]
pub fn rook_moves(sq: i32, blockers: u64) -> Bitboard {
    let idx = rook_index(sq, blockers);
    Bitboard(MAGIC_MOVES[idx])
}

#[must_use]
pub fn queen_moves(sq: i32, occ: u64) -> Bitboard {
    bishop_moves(sq, occ) | rook_moves(sq, occ)
}

#[must_use]
pub fn king_moves(sq: i32) -> Bitboard {
    Bitboard(KING_MASKS[sq as usize])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directions() {
        assert!(north(0x1000000) == 0x100000000);
        assert!(east(0x1000000) == 0x2000000);
        assert!(west(0x1000000) == 0x0);
        assert!(south(0x1000000) == 0x10000);
        assert!(north(0x80000000) == 0x8000000000);
        assert!(east(0x80000000) == 0x0);
        assert!(west(0x80000000) == 0x40000000);
        assert!(south(0x80000000) == 0x800000);
    }

    #[test]
    fn test_bishop_index() {
        assert_eq!(bishop_index(0, 0x0), 5378);
        assert_eq!(bishop_index(1, 0x0), 4093);
        assert_eq!(bishop_index(2, 0x0), 4314);
        assert_eq!(bishop_index(3, 0x0), 6587);
        assert_eq!(bishop_index(0, 0x1000000000), 5881);
        assert_eq!(bishop_index(28, 0x440000004400), 55380);
        assert_eq!(bishop_index(28, 0x100044001000), 55365);
    }

    #[test]
    fn test_rook_index() {
        assert_eq!(rook_index(0, 0x0), 26304);
        assert_eq!(rook_index(1, 0x0), 35520);
        assert_eq!(rook_index(2, 0x0), 38592);
        assert_eq!(rook_index(3, 0x0), 8026);
        assert_eq!(rook_index(0, 0x1000000000), 26304);
        assert_eq!(rook_index(28, 0x440000004400), 84966);
        assert_eq!(rook_index(28, 0x100044001000), 85547);
    }

    #[test]
    fn test_magics() {
        let tests = [
            (0, 0x808f70808080808),
            (1, 0x808f40800000000),
            (2, 0x0),
            (100, 0x808340808080000),
            (101, 0x8360808080800),
            (102, 0x8360808080808),
        ];

        for (idx, blockers) in tests {
            println!("idx:{idx} blockers:{blockers}");
            assert_eq!(MAGIC_MOVES[idx], blockers);
        }
    }

    #[test]
    fn test_magic_bishop() {
        assert_eq!(bishop_moves(0, 0x0).0, 0x8040201008040200);
        assert_eq!(bishop_moves(0, 0x1000000000).0, 0x1008040200);
        assert_eq!(bishop_moves(28, 0x440000004400).0, 0x442800284400);
        assert_eq!(bishop_moves(4, 0x2800).0, 0x2800);
    }

    #[test]
    fn test_magic_rook() {
        assert_eq!(rook_moves(0, 0x0).0, 0x1010101010101fe);
    }
}
