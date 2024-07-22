use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    bindgen::builder()
        .layout_tests(false)
        .header("wrapper.h")
        .clang_arg("-Iantichess-tb-api/src")
        .allowlist_function("antichess_tb_init")
        .allowlist_function("antichess_tb_add_path")
        .allowlist_function("antichess_tb_probe_dtw")
        .generate()
        .unwrap()
        .write_to_file(out_dir.join("bindings.rs"))
        .unwrap();

    cc::Build::new()
        .cpp(true)
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
        .compile("libantichesstb.a");

    println!("cargo:root={}", out_dir.display());
    println!(
        "cargo:include={}",
        env::current_dir()
            .unwrap()
            .join("antichess-tb-api")
            .display()
    );
}
