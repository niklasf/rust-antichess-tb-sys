use std::ffi::c_int;

use antichess_tb_sys::antichess_tb_init;

struct Test {
    fen: &'static str,
    dtw: c_int,
    res: c_int,
}

#[test]
fn test_reference() {
    assert_eq!(unsafe { antichess_tb_init() }, 0);

    let path = "an0";
    assert_eq!(
        unsafe { antichess_tb_add_path(path.as_ptr(), path.len()) },
        0
    );

    let tests = &[Test {
        fen: "7k/8/5K2/8/8/1R6/P7/8 w - - 0 1",
        dtw: 50,
        res: 0,
    }];

    for test in tests {}
}
