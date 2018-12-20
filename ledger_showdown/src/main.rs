extern crate byteorder;
extern crate rocksdb;
use byteorder::{BigEndian, ByteOrder};
use rand::prelude::*;
use rocksdb::{Options, DB};
//use std::sync::Arc;
//use std::thread;
use std::time::{Duration, Instant};

fn per_second(x: u64, elapsed: &Duration) -> f64 {
    let nanos = elapsed.as_secs() * 1_000_000_000 + elapsed.subsec_nanos() as u64;
    (x * 1_000_000_000) as f64 / nanos as f64
}

fn main() {
    // rocksdb
    let _ignored = DB::destroy(&Options::default(), "test");

    let mut options = Options::default();
    options.create_if_missing(true);

    let db = DB::open(&options, "test").expect("first open");

    const INIT: u64 = 100_000;
    const BLOCK: u64 = 100;
    const WRITES: u64 = 10_000;

    // initialize to a big ass db
    let data = [0u8; 1024 * 64];
    for i in 0..INIT {
        let mut key = [0u8; 16];
        BigEndian::write_u64(&mut key, i);
        db.put(&key, &data).expect(&format!("writing {} failed", i));
    }

    let mut rng = rand::thread_rng();
    // random order for each chunk of a BLOCK
    let now = Instant::now();
    for i in 0..WRITES / BLOCK {
        let mut keys: Vec<u64> = (i..i + BLOCK).collect();
        keys.shuffle(&mut rng);
        for i in keys {
            let mut key = [0u8; 16];
            BigEndian::write_u64(&mut key, i + INIT);
            db.put(&key, &data).expect(&format!("writing {} failed", i));
        }
    }

    println!(
        "rocksdb validator-style writes/sec {}",
        per_second(WRITES, &now.elapsed())
    );

    // append-style
    let now = Instant::now();
    for i in 0..WRITES {
        let mut key = [0u8; 16];
        BigEndian::write_u64(&mut key, i + INIT + WRITES);
        db.put(&key, &data).expect(&format!("writing {} failed", i));
    }
    println!(
        "rocksdb broadcast-style writes/sec {}",
        per_second(WRITES, &now.elapsed())
    );

    // rocksdb
    let _ignored = DB::destroy(&Options::default(), "test");
}
