#![feature(test)]

extern crate test;

use std::{fs::File, os::unix::prelude::FileExt};
use test::Bencher;

#[path = "../src/storage.rs"]
mod storage;

#[bench]
fn bench_read_4000_bytes(b: &mut Bencher) {
    let file = File::open("db.dat").unwrap();
    let mut buf = vec![0; 4000];
    b.iter(|| file.read_at(&mut buf, 0).unwrap());
}

#[bench]
fn bench_read_100_bytes(b: &mut Bencher) {
    let file = File::open("db.dat").unwrap();
    let mut buf = vec![0; 100];
    b.iter(|| file.read_at(&mut buf, 0).unwrap());
}

#[bench]
fn bench_get_definition(b: &mut Bencher) {
    let store = storage::Storage::new();
    b.iter(|| {
        store.get_definition("monitor".to_string());
    })
}
