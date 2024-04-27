use super::square::Square;
use crate::chess::bitboard::Bitboard;
use crate::chess::colour::Colour;
use crate::chess::piece::Piece;
use crate::chess::position::Position;
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
                    _ => panic!("Uh oh"),
                }
            }
        }

        // Side to move
        let should_flip = if let Some(part) = parts.next() {
            match part {
                "w" | "W" => false,
                "b" | "B" => true,
                _ => panic!("Uh oh"),
            }
        } else {
            panic!("Uh oh");
        };

        // Castling permissions
        if let Some(part) = parts.next() {
            for c in part.chars() {
                let (side, file, is_ksc) = match c {
                    'K' => {
                        let file = (self.get_white() & self.get_rooks() & Bitboard(0xFF))
                            .hsb()
                            .file() as u8;
                        (Colour::White, file, true)
                    }
                    'Q' => {
                        let file = (self.get_white() & self.get_rooks() & Bitboard(0xFF))
                            .lsb()
                            .file() as u8;
                        (Colour::White, file, false)
                    }
                    'k' => {
                        let file =
                            (self.get_black() & self.get_rooks() & Bitboard(0xFF00000000000000))
                                .hsb()
                                .file() as u8;
                        (Colour::Black, file, true)
                    }
                    'q' => {
                        let file =
                            (self.get_black() & self.get_rooks() & Bitboard(0xFF00000000000000))
                                .lsb()
                                .file() as u8;
                        (Colour::Black, file, false)
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
                    _ => panic!("Uh oh"),
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
            if part != "-" {
                let c1 = part.chars().nth(0).unwrap();
                let c2 = part.chars().nth(1).unwrap();
                let file = c1 as u8 - b'a';
                let rank = c2 as u8 - b'1';
                let idx = 8 * rank + file;
                self.ep = Some(Square(idx));
            }
        } else {
            panic!("Uh oh");
        };

        // Halfmove counter
        if let Some(part) = parts.next() {
            match part.parse::<i32>() {
                Ok(value) => self.halfmoves = value,
                Err(e) => panic!("{e}"),
            }
        } else {
            panic!("Uh oh");
        };

        // Fullmove counter
        if let Some(part) = parts.next() {
            match part.parse::<i32>() {
                Ok(value) => self.fullmoves = value,
                Err(e) => panic!("{e}"),
            }
        } else {
            panic!("Uh oh");
        };

        // Flip if required
        if should_flip {
            self.flip();
            self.turn = Colour::Black
        }

        // Set initial hash
        self.hash = self.calculate_hash();
    }
}
