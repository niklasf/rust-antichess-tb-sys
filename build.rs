use std::{env, path::PathBuf};

use tap::Pipe as _;

fn has_target_feature(feature: &str) -> bool {
    env::var("CARGO_CFG_TARGET_FEATURE")
        .unwrap()
        .split(',')
        .any(|f| f == feature)
}

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
        .std("gnu99")
        .flag_if_supported("-Wno-sign-compare")
        .flag_if_supported("-Wno-discarded-qualifiers")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-function")
        // src/tb/egtb/dictzip
        .pipe(|b| {
            if std::env::var_os("CARGO_CFG_UNIX").is_some() {
                b.define("HAVE_UNISTD_H", None)
            } else {
                b
            }
        })
        .define("HAVE_MMAP", None)
        .include("antichess-tb-api/src/tb/egtb/dictzip")
        .file("antichess-tb-api/src/tb/egtb/dictzip/data.c")
        .file("antichess-tb-api/src/tb/egtb/dictzip/dictzip.c")
        .file("antichess-tb-api/src/tb/egtb/dictzip/dz.c")
        // src/tb/egtb/dictzip/zlib
        .include("antichess-tb-api/src/tb/egtb/dictzip/zlib")
        .file("antichess-tb-api/src/tb/egtb/dictzip/zlib/adler32.c")
        .file("antichess-tb-api/src/tb/egtb/dictzip/zlib/compress.c")
        .file("antichess-tb-api/src/tb/egtb/dictzip/zlib/crc32.c")
        .file("antichess-tb-api/src/tb/egtb/dictzip/zlib/deflate.c")
        .file("antichess-tb-api/src/tb/egtb/dictzip/zlib/gzclose.c")
        .file("antichess-tb-api/src/tb/egtb/dictzip/zlib/gzlib.c")
        .file("antichess-tb-api/src/tb/egtb/dictzip/zlib/gzread.c")
        .file("antichess-tb-api/src/tb/egtb/dictzip/zlib/gzwrite.c")
        .file("antichess-tb-api/src/tb/egtb/dictzip/zlib/infback.c")
        .file("antichess-tb-api/src/tb/egtb/dictzip/zlib/inffast.c")
        .file("antichess-tb-api/src/tb/egtb/dictzip/zlib/inflate.c")
        .file("antichess-tb-api/src/tb/egtb/dictzip/zlib/inftrees.c")
        .file("antichess-tb-api/src/tb/egtb/dictzip/zlib/trees.c")
        .file("antichess-tb-api/src/tb/egtb/dictzip/zlib/uncompr.c")
        .file("antichess-tb-api/src/tb/egtb/dictzip/zlib/zutil.c")
        .compile("dictzip");

    cc::Build::new()
        .cpp(true)
        .std("c++17")
        .flag_if_supported("-Wno-unused-parameter")
        // src/tb
        .include("antichess-tb-api/src")
        .define("ANTI", None)
        .pipe(|b| {
            if has_target_feature("popcnt") {
                b.define("USE_POPCNT", None)
            } else {
                b
            }
        })
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
        .compile("antichesstb");

    println!("cargo:root={}", out_dir.display());
    println!(
        "cargo:include={}",
        env::current_dir()
            .unwrap()
            .join("antichess-tb-api")
            .display()
    );
}
