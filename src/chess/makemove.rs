use crate::chess::bitboard::Bitboard;
use crate::chess::mv::Mv;
use crate::chess::piece::Piece;
use crate::chess::position::Position;
use crate::chess::side::Side;
use crate::chess::square::{Square, *};

impl Position {
    pub fn makemove<const UPDATE_HASH: bool>(&mut self, mv: &Mv) {
        debug_assert!(mv.from.0 < 64);
        debug_assert!(mv.to.0 < 64);
        debug_assert!(mv.from.0 != mv.to.0);

        let bb_from = Bitboard::from_square(mv.from);
        let bb_to = Bitboard::from_square(mv.to);
        let piece = self.get_piece_on(mv.from);
        let captured = self.get_piece_on(mv.to);

        let ksq_us = (self.get_us() & self.get_kings()).lsb();
        let ksq_them = (self.get_them() & self.get_kings()).lsb();
        let ksc_us = Square::from_coords(self.castle_files[0], 0);
        let qsc_us = Square::from_coords(self.castle_files[1], 0);
        let ksc_them = Square::from_coords(self.castle_files[2], 7);
        let qsc_them = Square::from_coords(self.castle_files[3], 7);

        debug_assert!(piece.is_some());
        debug_assert!(captured.unwrap_or(Piece::None) != Piece::King);
        debug_assert!(mv.promo != Piece::Pawn);
        debug_assert!(mv.promo != Piece::King);

        if UPDATE_HASH {
            self.hash = self.predict_hash(mv);
        }

        // Relocate the piece
        self.colours[Side::Us as usize] ^= bb_from | bb_to;
        self.pieces[piece.unwrap() as usize] ^= bb_from | bb_to;

        self.halfmoves += 1;

        // Capture
        if self.get_them().is_set(mv.to) {
            self.colours[Side::Them as usize] ^= bb_to;
            self.pieces[captured.unwrap() as usize] ^= bb_to;
            self.halfmoves = 0;
        }

        // Pawn move
        if piece.unwrap() == Piece::Pawn {
            self.halfmoves = 0;
        }

        // En passant
        if piece.unwrap() == Piece::Pawn && mv.from.file() != mv.to.file() && captured.is_none() {
            self.colours[Side::Them as usize] ^= Bitboard::from_square(self.ep.unwrap()).south();
            self.pieces[Piece::Pawn as usize] ^= Bitboard::from_square(self.ep.unwrap()).south();
        }

        // Double pawn move
        if piece.unwrap() == Piece::Pawn && mv.to.0 - mv.from.0 == 16 {
            self.ep = Some(Square(mv.to.0 - 8));
        } else {
            self.ep = None;
        }

        // King side castle
        if (self.get_kings() & self.get_rooks()).is_occupied() && mv.to.0 > mv.from.0 {
            debug_assert!(piece.unwrap() == Piece::King);
            debug_assert!(captured.unwrap() == Piece::Rook);

            let ksc_sq = Square::from_coords(self.castle_files[0], 0);
            self.colours[Side::Us as usize] ^= bb_from | bb_to;
            self.pieces[Piece::King as usize] ^= bb_from | bb_to;

            // King
            self.colours[Side::Us as usize] ^= Bitboard::from_square(mv.from);
            self.colours[Side::Us as usize] ^= Bitboard::from_square(G1);
            self.pieces[Piece::King as usize] ^= Bitboard::from_square(mv.from);
            self.pieces[Piece::King as usize] ^= Bitboard::from_square(G1);

            // Rook
            self.colours[Side::Us as usize] ^= Bitboard::from_square(ksc_sq);
            self.colours[Side::Us as usize] ^= Bitboard::from_square(F1);
            self.pieces[Piece::Rook as usize] ^= Bitboard::from_square(ksc_sq);
            self.pieces[Piece::Rook as usize] ^= Bitboard::from_square(F1);
        }
        // Queen side castle
        else if (self.get_kings() & self.get_rooks()).is_occupied() && mv.to.0 < mv.from.0 {
            debug_assert!(piece.unwrap() == Piece::King);
            debug_assert!(captured.unwrap() == Piece::Rook);

            let qsc_sq = Square::from_coords(self.castle_files[1], 0);
            self.colours[Side::Us as usize] ^= bb_from | bb_to;
            self.pieces[Piece::King as usize] ^= bb_from | bb_to;

            // King
            self.colours[Side::Us as usize] ^= Bitboard::from_square(mv.from);
            self.colours[Side::Us as usize] ^= Bitboard::from_square(C1);
            self.pieces[Piece::King as usize] ^= Bitboard::from_square(mv.from);
            self.pieces[Piece::King as usize] ^= Bitboard::from_square(C1);

            // Rook
            self.colours[Side::Us as usize] ^= Bitboard::from_square(qsc_sq);
            self.colours[Side::Us as usize] ^= Bitboard::from_square(D1);
            self.pieces[Piece::Rook as usize] ^= Bitboard::from_square(qsc_sq);
            self.pieces[Piece::Rook as usize] ^= Bitboard::from_square(D1);
        }

        // Promo
        if mv.promo != Piece::None {
            self.pieces[Piece::Pawn as usize] ^= bb_to;
            self.pieces[mv.promo as usize] ^= bb_to;
        }

        // Castling permissions
        self.us_ksc &= mv.from != ksq_us && mv.from != ksc_us && mv.to != ksc_us;
        self.us_qsc &= mv.from != ksq_us && mv.from != qsc_us && mv.to != qsc_us;
        self.them_ksc &= mv.from != ksq_them && mv.from != ksc_them && mv.to != ksc_them;
        self.them_qsc &= mv.from != ksq_them && mv.from != qsc_them && mv.to != qsc_them;

        self.flip();

        #[cfg(debug_assertions)]
        if UPDATE_HASH {
            debug_assert_eq!(self.hash, self.calculate_hash());
        }
    }
}
