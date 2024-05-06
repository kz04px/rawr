use crate::chess::bitboard::Bitboard;
use crate::chess::square::Square;

#[must_use]
pub fn ray_ne(sq: Square, blockers: Bitboard) -> Bitboard {
    let mut mask = Bitboard::empty();

    mask |= Bitboard::from_square(sq).north_east();
    mask |= (mask & !blockers).north_east();
    mask |= (mask & !blockers).north_east();
    mask |= (mask & !blockers).north_east();
    mask |= (mask & !blockers).north_east();
    mask |= (mask & !blockers).north_east();
    mask |= (mask & !blockers).north_east();

    mask
}

#[must_use]
pub fn ray_nw(sq: Square, blockers: Bitboard) -> Bitboard {
    let mut mask = Bitboard::empty();

    mask |= Bitboard::from_square(sq).north_west();
    mask |= (mask & !blockers).north_west();
    mask |= (mask & !blockers).north_west();
    mask |= (mask & !blockers).north_west();
    mask |= (mask & !blockers).north_west();
    mask |= (mask & !blockers).north_west();
    mask |= (mask & !blockers).north_west();

    mask
}

#[must_use]
pub fn ray_se(sq: Square, blockers: Bitboard) -> Bitboard {
    let mut mask = Bitboard::empty();

    mask |= Bitboard::from_square(sq).south_east();
    mask |= (mask & !blockers).south_east();
    mask |= (mask & !blockers).south_east();
    mask |= (mask & !blockers).south_east();
    mask |= (mask & !blockers).south_east();
    mask |= (mask & !blockers).south_east();
    mask |= (mask & !blockers).south_east();

    mask
}

#[must_use]
pub fn ray_sw(sq: Square, blockers: Bitboard) -> Bitboard {
    let mut mask = Bitboard::empty();

    mask |= Bitboard::from_square(sq).south_west();
    mask |= (mask & !blockers).south_west();
    mask |= (mask & !blockers).south_west();
    mask |= (mask & !blockers).south_west();
    mask |= (mask & !blockers).south_west();
    mask |= (mask & !blockers).south_west();
    mask |= (mask & !blockers).south_west();

    mask
}

#[must_use]
pub fn ray_n(sq: Square, blockers: Bitboard) -> Bitboard {
    let mut mask = Bitboard::empty();

    mask |= Bitboard::from_square(sq).north();
    mask |= (mask & !blockers).north();
    mask |= (mask & !blockers).north();
    mask |= (mask & !blockers).north();
    mask |= (mask & !blockers).north();
    mask |= (mask & !blockers).north();
    mask |= (mask & !blockers).north();

    mask
}

#[must_use]
pub fn ray_s(sq: Square, blockers: Bitboard) -> Bitboard {
    let mut mask = Bitboard::empty();

    mask |= Bitboard::from_square(sq).south();
    mask |= (mask & !blockers).south();
    mask |= (mask & !blockers).south();
    mask |= (mask & !blockers).south();
    mask |= (mask & !blockers).south();
    mask |= (mask & !blockers).south();
    mask |= (mask & !blockers).south();

    mask
}

#[must_use]
pub fn ray_e(sq: Square, blockers: Bitboard) -> Bitboard {
    let mut mask = Bitboard::empty();

    mask |= Bitboard::from_square(sq).east();
    mask |= (mask & !blockers).east();
    mask |= (mask & !blockers).east();
    mask |= (mask & !blockers).east();
    mask |= (mask & !blockers).east();
    mask |= (mask & !blockers).east();
    mask |= (mask & !blockers).east();

    mask
}

#[must_use]
pub fn ray_w(sq: Square, blockers: Bitboard) -> Bitboard {
    let mut mask = Bitboard::empty();

    mask |= Bitboard::from_square(sq).west();
    mask |= (mask & !blockers).west();
    mask |= (mask & !blockers).west();
    mask |= (mask & !blockers).west();
    mask |= (mask & !blockers).west();
    mask |= (mask & !blockers).west();
    mask |= (mask & !blockers).west();

    mask
}

#[must_use]
pub fn knights(bb: Bitboard) -> Bitboard {
    bb.north().north_east()
        | bb.north().north_west()
        | bb.south().south_east()
        | bb.south().south_west()
        | bb.east().north_east()
        | bb.east().south_east()
        | bb.west().north_west()
        | bb.west().south_west()
}

#[must_use]
pub fn pawns<const US: bool>(bb: Bitboard) -> Bitboard {
    if US {
        bb.north_east() | bb.north_west()
    } else {
        bb.south_east() | bb.south_west()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::{bitboard::Bitboard, square::*};

    #[test]
    fn test_pawns() {
        assert_eq!(
            pawns::<true>(Bitboard::from_index(SquareIdx::A1)),
            Bitboard(0x200)
        );
        assert_eq!(
            pawns::<true>(Bitboard::from_index(SquareIdx::H1)),
            Bitboard(0x4000)
        );
        assert_eq!(
            pawns::<true>(Bitboard::from_index(SquareIdx::E4)),
            Bitboard(0x2800000000)
        );
    }

    #[test]
    fn test_knights() {
        assert_eq!(
            knights(Bitboard::from_index(SquareIdx::A1)),
            Bitboard(0x20400)
        );
        assert_eq!(knights(Bitboard(0x80)), Bitboard(0x402000));
        assert_eq!(
            knights(Bitboard(0x100000000000000)),
            Bitboard(0x4020000000000)
        );
        assert_eq!(
            knights(Bitboard(0x8000000000000000)),
            Bitboard(0x20400000000000)
        );
    }

    #[test]
    fn test_ray_ne() {
        assert_eq!(
            ray_ne(Square::from_index(SquareIdx::A1), Bitboard::empty()),
            Bitboard(0x8040201008040200)
        );
        assert_eq!(
            ray_ne(Square::from_index(SquareIdx::A8), Bitboard::empty()),
            Bitboard(0x0)
        );
        assert_eq!(
            ray_ne(Square::from_index(SquareIdx::H1), Bitboard::empty()),
            Bitboard(0x0)
        );
        assert_eq!(
            ray_ne(Square::from_index(SquareIdx::H8), Bitboard::empty()),
            Bitboard(0x0)
        );
    }

    #[test]
    fn test_ray_nw() {
        assert_eq!(
            ray_nw(Square::from_index(SquareIdx::A1), Bitboard::empty()),
            Bitboard(0x0)
        );
        assert_eq!(
            ray_nw(Square::from_index(SquareIdx::A8), Bitboard::empty()),
            Bitboard(0x0)
        );
        assert_eq!(
            ray_nw(Square::from_index(SquareIdx::H1), Bitboard::empty()),
            Bitboard(0x102040810204000)
        );
        assert_eq!(
            ray_nw(Square::from_index(SquareIdx::H8), Bitboard::empty()),
            Bitboard(0x0)
        );
    }

    #[test]
    fn test_ray_se() {
        assert_eq!(
            ray_se(Square::from_index(SquareIdx::A1), Bitboard::empty()),
            Bitboard(0x0)
        );
        assert_eq!(
            ray_se(Square::from_index(SquareIdx::A8), Bitboard::empty()),
            Bitboard(0x2040810204080)
        );
        assert_eq!(
            ray_se(Square::from_index(SquareIdx::H1), Bitboard::empty()),
            Bitboard(0x0)
        );
        assert_eq!(
            ray_se(Square::from_index(SquareIdx::H8), Bitboard::empty()),
            Bitboard(0x0)
        );
    }

    #[test]
    fn test_ray_sw() {
        assert_eq!(
            ray_sw(Square::from_index(SquareIdx::A1), Bitboard::empty()),
            Bitboard(0x0)
        );
        assert_eq!(
            ray_sw(Square::from_index(SquareIdx::A8), Bitboard::empty()),
            Bitboard(0x0)
        );
        assert_eq!(
            ray_sw(Square::from_index(SquareIdx::H1), Bitboard::empty()),
            Bitboard(0x0)
        );
        assert_eq!(
            ray_sw(Square::from_index(SquareIdx::H8), Bitboard::empty()),
            Bitboard(0x40201008040201)
        );
    }
}
