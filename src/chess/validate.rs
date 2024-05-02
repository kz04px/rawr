use crate::chess::bitboard::Bitboard;
use crate::chess::position::Position;
use crate::chess::side::Side;
use crate::chess::square::Square;

impl Position {
    #[must_use]
    pub fn validate(&self) -> Result<(), &'static str> {
        // Pawns in the right places
        if (self.get_pawns() & Bitboard(0xFF000000000000FF)).is_occupied() {
            return Err("Pawns on ranks 1 or 8");
        }

        // Colour overlaps
        if (self.get_white() & self.get_black()).is_occupied() {
            return Err("Colour bitboard overlap");
        }

        // Piece overlaps
        if (self.get_pawns() & self.get_knights()).is_occupied() {
            return Err("pawn knight overlap");
        } else if (self.get_pawns() & self.get_bishops()).is_occupied() {
            return Err("pawn bishop overlap");
        } else if (self.get_pawns() & self.get_rooks()).is_occupied() {
            return Err("pawn rook overlap");
        } else if (self.get_pawns() & self.get_queens()).is_occupied() {
            return Err("pawn queen overlap");
        } else if (self.get_pawns() & self.get_kings()).is_occupied() {
            return Err("pawn king overlap");
        } else if (self.get_knights() & self.get_bishops()).is_occupied() {
            return Err("knight bishop overlap");
        } else if (self.get_knights() & self.get_rooks()).is_occupied() {
            return Err("knight rook overlap");
        } else if (self.get_knights() & self.get_queens()).is_occupied() {
            return Err("knight queen overlap");
        } else if (self.get_knights() & self.get_kings()).is_occupied() {
            return Err("knight king overlap");
        } else if (self.get_bishops() & self.get_rooks()).is_occupied() {
            return Err("bishop rook overlap");
        } else if (self.get_bishops() & self.get_queens()).is_occupied() {
            return Err("bishop queen overlap");
        } else if (self.get_bishops() & self.get_kings()).is_occupied() {
            return Err("bishop king overlap");
        } else if (self.get_rooks() & self.get_queens()).is_occupied() {
            return Err("rook queen overlap");
        } else if (self.get_rooks() & self.get_kings()).is_occupied() {
            return Err("rook king overlap");
        } else if (self.get_queens() & self.get_kings()).is_occupied() {
            return Err("queen king overlap");
        }

        // En passant
        if let Some(ep) = self.ep {
            let bb = Bitboard::from_square(ep);

            // Must be on the opponent and on the right rank
            if ep.rank() != 5 {
                return Err("EP square not on 6th rank");
            }
            // Must be a pawn below
            else if (bb.south() & self.get_them() & self.get_pawns()).is_empty() {
                return Err("No pawn to create EP square");
            }
            // EP square must be empty
            else if (bb & self.get_occupied()).is_occupied() {
                return Err("EP square must be empty");
            }
        }

        // Piece counts
        if (self.get_white() & self.get_kings()).count() != 1 {
            return Err("Must be one white king");
        } else if (self.get_black() & self.get_kings()).count() != 1 {
            return Err("Must be one black king");
        }

        // Counters
        if self.halfmoves < 0 {
            return Err("halfmove counter out of range");
        } else if self.fullmoves < 1 {
            return Err("fullmoves counter out of range");
        }

        let us_ksq = (self.get_us() & self.get_kings()).lsb();
        let them_ksq = (self.get_them() & self.get_kings()).lsb();
        let us_rooks = self.get_us() & self.get_rooks();
        let them_rooks = self.get_them() & self.get_rooks();
        let us_ksc_sq = Square::from_coords(self.castle_files[0], 0);
        let us_qsc_sq = Square::from_coords(self.castle_files[1], 0);
        let them_ksc_sq = Square::from_coords(self.castle_files[2], 7);
        let them_qsc_sq = Square::from_coords(self.castle_files[3], 7);

        // Castling - King must be on its home rank
        if self.us_ksc && us_ksq.rank() != 0 {
            return Err("Castling permission while not on home rank");
        } else if self.us_qsc && us_ksq.rank() != 0 {
            return Err("Castling permission while not on home rank");
        } else if self.them_ksc && them_ksq.rank() != 7 {
            return Err("Castling permission while not on home rank");
        } else if self.them_qsc && them_ksq.rank() != 7 {
            return Err("Castling permission while not on home rank");
        }

        // Castling - Must have a rook where we expect one
        if self.us_ksc && !us_rooks.is_set(us_ksc_sq) {
            return Err("Castling rook missing");
        } else if self.us_qsc && !us_rooks.is_set(us_qsc_sq) {
            return Err("Castling rook missing");
        } else if self.them_ksc && !them_rooks.is_set(them_ksc_sq) {
            return Err("Castling rook missing");
        } else if self.them_qsc && !them_rooks.is_set(them_qsc_sq) {
            return Err("Castling rook missing");
        }

        // Can't capture opponent's king
        if self.is_sq_attacked(them_ksq, Side::Us) {
            return Err("Opponent king is capturable");
        }

        // Valid
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        let fens = [
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            "rnbqkb1r/pp2pp1p/3p1np1/8/3NP3/2N5/PPP2PPP/R1BQKB1R w KQkq - 0 6",
            "2rqr1k1/pp1bppbp/3p1np1/4n3/3NP3/1BN1BP2/PPPQ2PP/1K1R3R w - - 11 13",
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
            "bqnb1rkr/pp3ppp/3ppn2/2p5/5P2/P2P4/NPP1P1PP/BQ1BNRKR w HFhf - 2 9",
            "bbqr1krn/pppp1p1p/5n2/4p1p1/3P4/P3QP2/1PP1P1PP/BB1RNKRN w GDgd - 0 9",
            "qrkr2bb/pppppppp/8/1n2n3/1N5P/1P6/P1PPPPP1/QRKR1NBB w DBdb - 1 9",
            "4k3/8/8/8/8/8/8/4K3 w - - 0 1",
            "4k3/8/8/8/8/8/3p4/4K3 w - - 0 1",
        ];

        assert!(Position::startpos().validate().is_ok());
        for fen in fens {
            assert!(Position::from_fen(fen).validate().is_ok());
        }
    }

    #[test]
    fn invalid() {
        let fens = [
            // Empty board
            "8/8/8/8/8/8/8/8 w - - 0 1",
            // Not enough kings
            "8/8/8/8/8/8/8/4K3 w - - 0 1",
            "4k3/8/8/8/8/8/8/8 w - - 0 1",
            // Too many kings
            "4k3/8/8/8/8/8/8/1K2K3 w - - 0 1",
            "1k2k3/8/8/8/8/8/8/4K3 w - - 0 1",
            // Pawns on the 1st and 8th ranks
            "1P2k3/8/8/8/8/8/8/4K3 w - - 0 1",
            "4k3/8/8/8/8/8/8/1p2K3 w - - 0 1",
            "4k3/8/8/8/8/8/8/1P2K3 w - - 0 1",
            "1p2k3/8/8/8/8/8/8/4K3 w - - 0 1",
            // Counters
            "4k3/8/8/8/8/8/8/4K3 w - - -12 1",
            "4k3/8/8/8/8/8/8/4K3 w - - 0 0",
            "4k3/8/8/8/8/8/8/4K3 w - - 0 -12",
            // Incorrect castling
            "4k3/8/8/8/8/8/8/4K3 w K - 0 1",
            "4k3/8/8/8/8/8/8/R3K3 w K - 0 1",
            "4k3/8/8/8/8/8/8/4K3 w Q - 0 1",
            "4k3/8/8/8/8/8/8/4K2R w Q - 0 1",
            "4k3/8/8/8/8/8/8/4K3 w k - 0 1",
            "r3k3/8/8/8/8/8/8/4K3 w k - 0 1",
            "4k3/8/8/8/8/8/8/4K3 w q - 0 1",
            "4k2r/8/8/8/8/8/8/4K3 w q - 0 1",
            "4k3/8/8/8/8/8/4K3/R6R w KQ - 0 1",
            "r6r/4k3/8/8/8/8/8/4K3 w kq - 0 1",
            "8/r3k2r/8/8/8/8/R3K2R/8 w KQkq - 0 1",
            "8/r3k2r/8/8/8/8/R3K2R/8 w K - 0 1",
            "8/r3k2r/8/8/8/8/R3K2R/8 w Q - 0 1",
            "8/r3k2r/8/8/8/8/R3K2R/8 w k - 0 1",
            "8/r3k2r/8/8/8/8/R3K2R/8 w q - 0 1",
            // Capture king
            "4k3/5P2/8/8/8/8/8/4K3 w - - 0 1",
            "4k3/8/5N2/8/8/8/8/4K3 w - - 0 1",
            "4k3/8/6B1/8/8/8/8/4K3 w - - 0 1",
            "4k3/8/8/8/4R3/8/8/4K3 w - - 0 1",
            "4k3/8/8/8/4Q3/8/8/4K3 w - - 0 1",
            "4k3/4K3/8/8/8/8/8/8 w - - 0 1",
            // Illegal EP square
            "4k3/8/8/8/8/8/8/4K3 w - e6 0 1",
            "4k3/8/8/8/8/8/8/4K3 b - e3 0 1",
            "4k3/8/8/4P3/8/8/8/4K3 b - e4 0 1",
            "4k3/8/8/8/8/8/P7/4K3 w - a1 0 1",
        ];

        for fen in fens {
            let result = std::panic::catch_unwind(|| Position::from_fen(fen));
            println!("{}", fen);
            if let Ok(pos) = result {
                assert!(pos.validate().is_err());
            }
        }
    }
}
