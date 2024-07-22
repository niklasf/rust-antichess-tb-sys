#![no_main]

use std::{
    ffi::{c_char, c_int},
    sync::Once,
};

use antichess_tb_sys::{antichess_tb_add_path, antichess_tb_init, antichess_tb_probe_dtw};
use arbitrary::{Arbitrary, Unstructured};
use libfuzzer_sys::fuzz_target;

#[derive(Debug)]
struct Square(c_int);

impl Arbitrary<'_> for Square {
    fn arbitrary(u: &mut Unstructured<'_>) -> arbitrary::Result<Square> {
        u.int_in_range(0..=63).map(Square)
    }
}

#[derive(Debug)]
struct Role(c_int);

impl Arbitrary<'_> for Role {
    fn arbitrary(u: &mut Unstructured<'_>) -> arbitrary::Result<Role> {
        u.int_in_range(1..=6).map(Role)
    }
}

#[derive(Debug, Arbitrary)]
struct Pos {
    white: Vec<(Square, Role)>,
    black: Vec<(Square, Role)>,
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

    let (white_squares, white_pieces): (Vec<c_int>, Vec<c_int>) = pos
        .white
        .into_iter()
        .map(|(Square(s), Role(r))| (s, r))
        .unzip();
    let (black_squares, black_pieces): (Vec<c_int>, Vec<c_int>) = pos
        .black
        .into_iter()
        .map(|(Square(s), Role(r))| (s, r))
        .unzip();

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
            pos.ep_square.map_or(-1, |Square(s)| s),
            &mut dtw,
        );
    }
});
