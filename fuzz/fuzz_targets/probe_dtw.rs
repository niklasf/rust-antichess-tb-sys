#![no_main]

use std::{
    collections::HashMap,
    ffi::{c_char, c_int},
    sync::Once,
};

use antichess_tb_sys::{antichess_tb_add_path, antichess_tb_init, antichess_tb_probe_dtw};
use arbitrary::{Arbitrary, Unstructured};
use libfuzzer_sys::fuzz_target;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Square(c_int);

impl Square {
    fn from_file_and_rank(file: c_int, rank: c_int) -> Option<Square> {
        if 0 <= file && file < 8 && 0 <= rank && rank < 8 {
            Some(Square(file + rank * 8))
        } else {
            None
        }
    }

    fn rank(self) -> c_int {
        self.0 / 8
    }

    fn file(self) -> c_int {
        self.0 % 8
    }

    fn pawn_pushed_from(self) -> Square {
        Square(self.0 ^ 24)
    }

    fn pawn_pushed_to(self) -> Square {
        Square(self.0 ^ 8)
    }

    fn left(self) -> Option<Square> {
        Square::from_file_and_rank(self.file() - 1, self.rank())
    }

    fn right(self) -> Option<Square> {
        Square::from_file_and_rank(self.file() - 1, self.rank())
    }
}

impl Arbitrary<'_> for Square {
    fn arbitrary(u: &mut Unstructured<'_>) -> arbitrary::Result<Square> {
        u.int_in_range(0..=63).map(Square)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Role(c_int);

impl Arbitrary<'_> for Role {
    fn arbitrary(u: &mut Unstructured<'_>) -> arbitrary::Result<Role> {
        u.int_in_range(1..=6).map(Role)
    }
}

#[derive(Debug, Arbitrary, Eq, PartialEq)]
struct Piece {
    role: Role,
    white: bool,
}

impl Piece {
    fn pawn(white: bool) -> Piece {
        Piece {
            role: Role(1),
            white,
        }
    }
}

#[derive(Debug, Arbitrary)]
struct Pos {
    board: HashMap<Square, Piece>,
    white_to_move: bool,
    ep_square: Option<Square>,
}

fuzz_target!(|pos: Pos| {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        assert_eq!(unsafe { antichess_tb_init() }, 0, "init");

        let path = "a0";
        assert_eq!(
            unsafe { antichess_tb_add_path(path.as_ptr() as *const c_char, path.len()) },
            0,
            "add path"
        );
    });

    let mut white_squares: Vec<c_int> = Vec::new();
    let mut white_pieces: Vec<c_int> = Vec::new();
    let mut black_squares: Vec<c_int> = Vec::new();
    let mut black_pieces: Vec<c_int> = Vec::new();
    for (square, piece) in &pos.board {
        if piece.role == Role(1) && (square.rank() == 0 || square.rank() == 7) {
            continue; // Do not put pawn on backrank
        }

        if piece.white {
            white_squares.push(square.0);
            white_pieces.push(piece.role.0);
        } else {
            black_squares.push(square.0);
            black_pieces.push(piece.role.0);
        }
    }

    let legal_ep_square = pos.ep_square.filter(|s| {
        s.rank() == if pos.white_to_move { 5 } else { 2 }
            && !pos.board.contains_key(s)
            && !pos.board.contains_key(&s.pawn_pushed_from())
            && pos.board.get(&s.pawn_pushed_to()) == Some(&Piece::pawn(!pos.white_to_move))
            && (s.pawn_pushed_to().left().and_then(|c| pos.board.get(&c))
                == Some(&Piece::pawn(pos.white_to_move))
                || s.pawn_pushed_to().right().and_then(|c| pos.board.get(&c))
                    == Some(&Piece::pawn(pos.white_to_move)))
    });

    let mut dtw: c_int = 0;
    unsafe {
        antichess_tb_probe_dtw(
            white_squares.as_ptr(),
            white_pieces.as_ptr(),
            white_squares.len(),
            black_squares.as_ptr(),
            black_pieces.as_ptr(),
            black_squares.len(),
            if pos.white_to_move { 0 } else { 1 },
            legal_ep_square.map_or(-1, |Square(s)| s),
            &mut dtw,
        );
    }
});
