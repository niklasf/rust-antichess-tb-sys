use std::ffi::{c_char, c_int};

use antichess_tb_sys::{antichess_tb_add_path, antichess_tb_init, antichess_tb_probe_dtw};
use shakmaty::{fen::Fen, ByColor};

#[test]
fn test_reference() {
    assert_eq!(unsafe { antichess_tb_init() }, 0, "init");

    let path = "an0";
    let res = -unsafe { antichess_tb_add_path(path.as_ptr() as *const c_char, path.len()) };
    println!("{res} missing tables");

    let tests = [
        ("7k/8/5K2/8/8/1R6/P7/8 w - - 0 1", 50, 0),
        ("8/8/8/8/8/3k4/8/1R6 b - - 0 1", -35, 0),
        ("8/8/8/8/8/P7/8/7k b - - 0 1", -45, 0),
        ("8/8/K7/8/8/6b1/8/3b4 w - - 0 1", -47, 0),
        ("8/4p3/8/8/8/P7/P7/8 w - - 99 105", 68, 0),
        ("2Q2R2/8/8/8/8/8/8/1K5k w - - 0 1", 110, 0),
        ("8/1P6/8/8/8/K2P4/8/2k5 w - - 0 1", 142, 0),
        ("8/5N2/5B2/8/k7/8/8/2K5 w - - 0 1", 148, 1),
        ("8/8/8/8/8/5P2/1B6/2N3k1 w - - 0 1", 174, 1),
        ("8/8/8/8/8/5Pp1/3B4/6N1 w - - 0 1", 174, 1),
        ("8/8/8/5N2/8/B4P2/8/6k1 w - - 0 1", 162, 1),
        ("8/8/8/8/3N4/B4P2/8/6k1 b - - 0 1", -161, 1),
        ("8/5P2/8/8/3N4/B7/8/7k w - - 0 1", 72, 0),
        ("8/2NK4/8/8/8/B2k4/8/8 b - - 0 1", -105, 1),
        ("8/2NK4/8/8/8/B7/2k5/8 w - - 0 1", 104, 0),
        ("8/8/8/8/k7/8/8/K3K1N1 w - - 0 1", 104, 0),
        ("8/8/8/8/k7/8/4N3/K3K3 b - - 1 1", 0, 2),
        ("8/8/8/8/1k6/8/4N3/K3K3 w - - 0 1", 0, 2),
        ("8/p7/8/1P6/7p/7R/8/8 b - - 0 25", 4, 0),
        ("8/8/8/pP6/7p/7R/8/8 w - a6 0 26", -3, 0),
        ("8/8/8/pP6/8/5p2/1P6/8 w - a6 0 26", -11, 0),
        ("8/8/8/pP6/8/5p2/1P6/8 w - - 0 26", 0, 2),
        ("3N4/8/8/6Br/2pP4/8/8/1Q6 b - - 0 1", 0, -2),
    ];

    for (fen, ref_dtw, ref_res) in tests {
        let setup = fen.parse::<Fen>().unwrap().into_setup();

        let mut squares: ByColor<Vec<c_int>> = ByColor::default();
        let mut pieces: ByColor<Vec<c_int>> = ByColor::default();
        for (sq, piece) in setup.board {
            squares.get_mut(piece.color).push(c_int::from(sq));
            pieces.get_mut(piece.color).push(c_int::from(piece.role));
        }

        let mut dtw: c_int = 0;
        let res = unsafe {
            antichess_tb_probe_dtw(
                squares.white.as_ptr(),
                pieces.white.as_ptr(),
                squares.white.len(),
                squares.black.as_ptr(),
                pieces.black.as_ptr(),
                squares.black.len(),
                setup.turn.fold_wb(0, 1),
                setup.ep_square.map_or(-1, c_int::from),
                &mut dtw,
            )
        };
        assert_eq!((res, dtw), (ref_res, ref_dtw), "res, dtw for {}", fen);
    }
}
