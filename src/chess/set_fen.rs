use crate::chess::bitboard::Bitboard;
use crate::chess::colour::Colour;
use crate::chess::piece::Piece;
use crate::chess::position::Position;
use crate::chess::square::Square;
use core::panic;

impl Position {
    pub fn set_fen(&mut self, fen: &str) {
        if fen == "startpos" {
            self.set_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
            return;
        }

        *self = Position {
            colours: [Bitboard(0x0), Bitboard(0x0)],
            pieces: [
                Bitboard(0x0),
                Bitboard(0x0),
                Bitboard(0x0),
                Bitboard(0x0),
                Bitboard(0x0),
                Bitboard(0x0),
            ],
            halfmoves: 0,
            fullmoves: 1,
            turn: Colour::White,
            ep: None,
            us_ksc: false,
            us_qsc: false,
            them_ksc: false,
            them_qsc: false,
            castle_files: [7, 0, 7, 0],
            hash: 0u64,
            is_frc: self.is_frc,
        };

        let mut parts = fen.split(' ');

        // Board
        if let Some(part) = parts.next() {
            let mut idx = 0u8;

            for c in part.chars() {
                let rank: u8 = idx / 8;
                let file = idx % 8;
                let sq = Square(8 * (7 - rank) + file);
                let bb = Bitboard::from_square(sq);

                match c {
                    'P' => {
                        self.colours[Colour::White as usize] ^= bb;
                        self.pieces[Piece::Pawn as usize] ^= bb;
                        idx += 1;
                    }
                    'N' => {
                        self.colours[Colour::White as usize] ^= bb;
                        self.pieces[Piece::Knight as usize] ^= bb;
                        idx += 1;
                    }
                    'B' => {
                        self.colours[Colour::White as usize] ^= bb;
                        self.pieces[Piece::Bishop as usize] ^= bb;
                        idx += 1;
                    }
                    'R' => {
                        self.colours[Colour::White as usize] ^= bb;
                        self.pieces[Piece::Rook as usize] ^= bb;
                        idx += 1;
                    }
                    'Q' => {
                        self.colours[Colour::White as usize] ^= bb;
                        self.pieces[Piece::Queen as usize] ^= bb;
                        idx += 1;
                    }
                    'K' => {
                        self.colours[Colour::White as usize] ^= bb;
                        self.pieces[Piece::King as usize] ^= bb;
                        idx += 1;
                    }
                    'p' => {
                        self.colours[Colour::Black as usize] ^= bb;
                        self.pieces[Piece::Pawn as usize] ^= bb;
                        idx += 1;
                    }
                    'n' => {
                        self.colours[Colour::Black as usize] ^= bb;
                        self.pieces[Piece::Knight as usize] ^= bb;
                        idx += 1;
                    }
                    'b' => {
                        self.colours[Colour::Black as usize] ^= bb;
                        self.pieces[Piece::Bishop as usize] ^= bb;
                        idx += 1;
                    }
                    'r' => {
                        self.colours[Colour::Black as usize] ^= bb;
                        self.pieces[Piece::Rook as usize] ^= bb;
                        idx += 1;
                    }
                    'q' => {
                        self.colours[Colour::Black as usize] ^= bb;
                        self.pieces[Piece::Queen as usize] ^= bb;
                        idx += 1;
                    }
                    'k' => {
                        self.colours[Colour::Black as usize] ^= bb;
                        self.pieces[Piece::King as usize] ^= bb;
                        idx += 1;
                    }
                    '1' => idx += 1,
                    '2' => idx += 2,
                    '3' => idx += 3,
                    '4' => idx += 4,
                    '5' => idx += 5,
                    '6' => idx += 6,
                    '7' => idx += 7,
                    '8' => idx += 8,
                    '/' => {}
                    _ => panic!("Invalid FEN: unrecognised piece"),
                }
            }

            if idx != 64 {
                panic!("Invalid FEN: invalid number of pieces");
            }
        }

        // Side to move
        let should_flip = if let Some(part) = parts.next() {
            match part {
                "w" | "W" => false,
                "b" | "B" => true,
                _ => panic!("Invalid FEN: side to move illegal character"),
            }
        } else {
            panic!("Invalid FEN: side to move missing")
        };

        // Basic legality check
        if (self.get_white() & self.get_kings()).is_empty() {
            panic!("Invalid FEN: white king missing");
        } else if (self.get_black() & self.get_kings()).is_empty() {
            panic!("Invalid FEN: black king missing");
        }

        // Castling permissions
        if let Some(part) = parts.next() {
            let wksq = (self.get_white() & self.get_kings()).lsb();
            let bksq = (self.get_black() & self.get_kings()).lsb();
            let white_ks = Bitboard(0xFF) & Bitboard::ray_east(wksq);
            let white_qs = Bitboard(0xFF) & Bitboard::ray_west(wksq);
            let black_ks = Bitboard(0xFF00000000000000) & Bitboard::ray_east(bksq);
            let black_qs = Bitboard(0xFF00000000000000) & Bitboard::ray_west(bksq);

            for (idx, c) in part.chars().enumerate() {
                // Look for duplicates
                for i in 0..idx {
                    if part.chars().nth(i).unwrap() == c {
                        panic!("Invalid FEN: duplicated castling permissions");
                    }
                }

                let (side, file, is_ksc) = match c {
                    'K' => {
                        let rooks = self.get_white() & self.get_rooks() & white_ks;
                        if rooks.is_occupied() {
                            (Colour::White, rooks.hsb().file() as u8, true)
                        } else {
                            panic!("Invalid FEN: white king side castle rook missing");
                        }
                    }
                    'Q' => {
                        let rooks = self.get_white() & self.get_rooks() & white_qs;
                        if rooks.is_occupied() {
                            (Colour::White, rooks.lsb().file() as u8, false)
                        } else {
                            panic!("Invalid FEN: white queen side castle rook missing");
                        }
                    }
                    'k' => {
                        let rooks = self.get_black() & self.get_rooks() & black_ks;
                        if rooks.is_occupied() {
                            (Colour::Black, rooks.hsb().file() as u8, true)
                        } else {
                            panic!("Invalid FEN: black king side castle rook missing");
                        }
                    }
                    'q' => {
                        let rooks = self.get_black() & self.get_rooks() & black_qs;
                        if rooks.is_occupied() {
                            (Colour::Black, rooks.lsb().file() as u8, false)
                        } else {
                            panic!("Invalid FEN: black queen side castle rook missing");
                        }
                    }
                    'A'..='H' => {
                        let ksq = (self.get_kings() & self.get_us()).lsb();
                        let file = c as u8 - 'A' as u8;
                        (Colour::White, file, (file > ksq.file() as u8))
                    }
                    'a'..='h' => {
                        let ksq = (self.get_kings() & self.get_them()).lsb();
                        let file = c as u8 - 'a' as u8;
                        (Colour::Black, file, (file > ksq.file() as u8))
                    }
                    '-' => break,
                    _ => panic!("Invalid FEN: unrecognised castling permission"),
                };

                match (side, is_ksc) {
                    (Colour::White, true) => {
                        self.us_ksc = true;
                        self.castle_files[0] = file;
                    }
                    (Colour::White, false) => {
                        self.us_qsc = true;
                        self.castle_files[1] = file;
                    }
                    (Colour::Black, true) => {
                        self.them_ksc = true;
                        self.castle_files[2] = file;
                    }
                    (Colour::Black, false) => {
                        self.them_qsc = true;
                        self.castle_files[3] = file;
                    }
                }
            }
        }

        // En passant
        if let Some(part) = parts.next() {
            if part == "-" {
                self.ep = None;
            } else if part.len() == 2 {
                let c1 = part.chars().nth(0).unwrap();
                let c2 = part.chars().nth(1).unwrap();
                let file = c1 as u8 - b'a';
                let rank = c2 as u8 - b'1';
                let idx = 8 * rank + file;
                self.ep = Some(Square(idx));
            } else {
                panic!("Invalid FEN: illegal EP square");
            }
        } else {
            panic!("Invalid FEN: EP value missing");
        };

        // Halfmove counter
        if let Some(part) = parts.next() {
            match part.parse::<i32>() {
                Ok(value) => {
                    if value < 0 {
                        panic!("Invalid FEN: halfmove counter out of range");
                    } else {
                        self.halfmoves = value
                    }
                }
                Err(e) => panic!("{e}"),
            }
        } else {
            panic!("Invalid FEN: halfmove counter missing");
        };

        // Fullmove counter
        if let Some(part) = parts.next() {
            match part.parse::<i32>() {
                Ok(value) => {
                    if value < 0 {
                        panic!("Invalid FEN: fullmove counter out of range");
                    } else {
                        self.fullmoves = value
                    }
                }
                Err(e) => panic!("{e}"),
            }
        } else {
            panic!("Invalid FEN: fullmove counter missing");
        };

        // Extra string?
        if parts.next().is_some() {
            panic!("Invalid FEN: extra parts");
        }

        // Flip if required
        if should_flip {
            self.flip();
            self.turn = Colour::Black
        }

        // Set initial hash
        self.hash = self.calculate_hash();

        match self.validate() {
            Err(e) => panic!("Invalid FEN: {e}"),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        let fens = [
            "startpos",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            "rnbqkb1r/pp2pp1p/3p1np1/8/3NP3/2N5/PPP2PPP/R1BQKB1R w KQkq - 0 6",
            "2rqr1k1/pp1bppbp/3p1np1/4n3/3NP3/1BN1BP2/PPPQ2PP/1K1R3R w - - 11 13",
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 200 1",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 200",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b - - 0 1",
            "4k3/8/8/4p3/8/8/8/4K3 w - e6 0 1",
            "rnbqkbnr/pppppppp/qqqqqqqq/qqqqqqqq/QQQQQQQQ/QQQQQQQQ/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        ];

        assert!(Position::startpos().validate().is_ok());
        for fen in fens {
            let result = std::panic::catch_unwind(|| Position::from_fen(fen));
            assert!(result.is_ok());
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
            "4k3/8/8/8/8/8/8/4K3 w - a0 0 1",
            "4k3/8/8/8/8/8/8/4K3 w - z0 0 1",
            "4k3/8/8/8/8/8/8/4K3 w - a9 0 1",
            "4k3/8/8/8/8/8/8/4K3 w - z9 0 1",
            "4k3/8/4n3/4p3/8/8/8/4K3 w - e6 0 1",
            // Incomplete
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0",
            // Extra pieces
            "4k3/ppppppppp/8/8/8/8/PPPPPPPP/4K3 w - - 0 1",
            "4k3/8/8/8/8/8/PPPPPPPPP/4K3 w - - 0 1",
            "4k3/ppppppppp/8/8/8/8/8/4K3 w - - 0 1",
            "4k3/8/8/8/PPPPPPPPP/8/8/4K3 w - - 0 1",
            // Extra words
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 extra",
            // Illegal parts
            "test w KQkq - 0 1",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkk - 0 1",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR test KQkq - 0 1",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w test - 0 1",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq test 0 1",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - test 1",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 test",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR wwwww KQkq - 0 1",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KKKKKKQkq - 0 1",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq ---- 0 1",
            // Other
            "",
            "test string",
        ];

        for fen in fens {
            println!("{}", fen);
            let result = std::panic::catch_unwind(|| Position::from_fen(fen));
            assert!(result.is_err());
        }
    }
}
