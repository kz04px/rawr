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
        if captured.is_some() {
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
        if piece.unwrap() == Piece::King && mv.from == E1 && mv.to == G1 {
            self.colours[Side::Us as usize] ^= Bitboard(0xa0);
            self.pieces[Piece::Rook as usize] ^= Bitboard(0xa0);
        }
        // Queen side castle
        else if piece.unwrap() == Piece::King && mv.from == E1 && mv.to == C1 {
            self.colours[Side::Us as usize] ^= Bitboard(0x9);
            self.pieces[Piece::Rook as usize] ^= Bitboard(0x9);
        }

        // Promo
        if mv.promo != Piece::None {
            self.pieces[Piece::Pawn as usize] ^= bb_to;
            self.pieces[mv.promo as usize] ^= bb_to;
        }

        // Castling permissions
        self.us_ksc &= mv.from != E1 && mv.from != H1 && mv.to != H1;
        self.us_qsc &= mv.from != E1 && mv.from != A1 && mv.to != A1;
        self.them_ksc &= mv.from != E8 && mv.from != H8 && mv.to != H8;
        self.them_qsc &= mv.from != E8 && mv.from != A8 && mv.to != A8;

        self.flip();
    }
}
