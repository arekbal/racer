extern crate test;

use std::env::var;
use std::io::Read;
use std::fs::File;
use std::path::PathBuf;

use codeiter::StmtIndicesIter;
use codecleaner::code_chunks;
use scopes::{mask_comments, mask_sub_scopes};
use core::IndexedSource;

use self::test::Bencher;

fn get_rust_file_str(path: &[&str]) -> String {
    let mut src_path = match var("RUST_SRC_PATH") {
        Ok(env) => { PathBuf::from(&env) },
        _ => panic!("Cannot find $RUST_SRC_PATH")
    };
    for &s in path.iter() { src_path.push(s); }

    let mut s = String::new();
    File::open(&src_path).unwrap().read_to_string(&mut s).unwrap();
    s
}

#[bench]
fn bench_code_chunks(b: &mut Bencher) {
    let src = &get_rust_file_str(&["liballoc", "vec.rs"]);
    b.iter(|| {
        test::black_box(code_chunks(src).collect::<Vec<_>>());
    });
}

#[bench]
fn bench_iter_stmts(b: &mut Bencher) {
    let src = &get_rust_file_str(&["liballoc", "vec.rs"]);
    b.iter(|| {
        test::black_box(StmtIndicesIter::from_parts(src, code_chunks(src))
                        .collect::<Vec<_>>());
    });
}

#[bench]
fn bench_mask_comments(b: &mut Bencher) {
    let src_indexed = IndexedSource::new(get_rust_file_str(&["liballoc", "vec.rs"]));
    let src = src_indexed.as_src();
    b.iter(|| {
        test::black_box(mask_comments(src));
    });
}

#[bench]
fn bench_mask_sub_scopes(b: &mut Bencher) {
    let src = &get_rust_file_str(&["liballoc", "vec.rs"]);
    b.iter(|| {
        test::black_box(mask_sub_scopes(src));
    });
}

