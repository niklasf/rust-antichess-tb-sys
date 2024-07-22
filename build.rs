fn main() {
    cc::Build::new()
        .std("c++17")
        .flag_if_supported("-Wno-unused-parameter")
        // src/tb
        .include("antichess-tb-api/src")
        .define("ANTI", None)
        .file("antichess-tb-api/src/tb/benchmark.cpp")
        .file("antichess-tb-api/src/tb/bitbase.cpp")
        .file("antichess-tb-api/src/tb/bitboard.cpp")
        .file("antichess-tb-api/src/tb/endgame.cpp")
        .file("antichess-tb-api/src/tb/evaluate.cpp")
        .file("antichess-tb-api/src/tb/material.cpp")
        .file("antichess-tb-api/src/tb/misc.cpp")
        .file("antichess-tb-api/src/tb/movegen.cpp")
        .file("antichess-tb-api/src/tb/movepick.cpp")
        .file("antichess-tb-api/src/tb/pawns.cpp")
        .file("antichess-tb-api/src/tb/psqt.cpp")
        .file("antichess-tb-api/src/tb/position.cpp")
        .file("antichess-tb-api/src/tb/search.cpp")
        .file("antichess-tb-api/src/tb/thread.cpp")
        .file("antichess-tb-api/src/tb/timeman.cpp")
        .file("antichess-tb-api/src/tb/tt.cpp")
        .file("antichess-tb-api/src/tb/tune.cpp")
        .file("antichess-tb-api/src/tb/uci.cpp")
        .file("antichess-tb-api/src/tb/ucioption.cpp")
        // src/tb/syzygy
        .file("antichess-tb-api/src/tb/syzygy/tbprobe.cpp")
        // src/tb/egtb
        .file("antichess-tb-api/src/tb/egtb/elements.cpp")
        .file("antichess-tb-api/src/tb/egtb/tb_reader.cpp")
        .file("antichess-tb-api/src/tb/egtb/tb_idx.cpp")
        // src
        .file("antichess-tb-api/src/antichess_tb_api.cpp")
        .compile("libantichess-tb-api.a");
}
