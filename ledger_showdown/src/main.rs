extern crate byteorder;
use byteorder::{BigEndian, ByteOrder};
use rand::prelude::*;
//use std::sync::Arc;
//use std::thread;
use std::fs::{create_dir_all, remove_dir_all};
use std::time::{Duration, Instant};

fn per_second(x: u64, elapsed: &Duration) -> f64 {
    let nanos = elapsed.as_secs() * 1_000_000_000 + elapsed.subsec_nanos() as u64;
    (x * 1_000_000_000) as f64 / nanos as f64
}
const INIT: u64 = 100_000;
const BLOCK: u64 = 100;
const WRITES: u64 = 10_000;

fn rocksdb() {
    // rocksdb
    extern crate rocksdb;
    use rocksdb::{Options, DB};

    let _ignored = DB::destroy(&Options::default(), "rocksdb");

    let mut options = Options::default();
    options.create_if_missing(true);

    let db = DB::open(&options, "test").expect("first open");

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
    let _ignored = DB::destroy(&Options::default(), "rocksdb");
}

//fn sled() {
//    extern crate sled;
//    use sled::Tree;
//
//    let _ignored = remove_dir_all("sled");
//
//    let tree = Tree::start_default("sled").unwrap();
//
//    let data = [0u8; 1024 * 64];
//    for i in 0..INIT {
//        let mut key = [0u8; 16];
//        BigEndian::write_u64(&mut key, i);
//        tree.set(&key, data.to_vec())
//            .expect(&format!("writing {} failed", i));
//    }
//
//    let mut rng = rand::thread_rng();
//    // random order for each chunk of a BLOCK
//    let now = Instant::now();
//    for i in 0..WRITES / BLOCK {
//        let mut keys: Vec<u64> = (i..i + BLOCK).collect();
//        keys.shuffle(&mut rng);
//        for i in keys {
//            let mut key = [0u8; 16];
//            BigEndian::write_u64(&mut key, i + INIT);
//            tree.set(&key, data.to_vec())
//                .expect(&format!("writing {} failed", i));
//        }
//    }
//
//    println!(
//        "sled validator-style writes/sec {}",
//        per_second(WRITES, &now.elapsed())
//    );
//
//    // append-style
//    let now = Instant::now();
//    for i in 0..WRITES {
//        let mut key = [0u8; 16];
//        BigEndian::write_u64(&mut key, i + INIT + WRITES);
//        tree.set(&key, data.to_vec())
//            .expect(&format!("writing {} failed", i));
//    }
//    println!(
//        "sled broadcast-style writes/sec {}",
//        per_second(WRITES, &now.elapsed())
//    );
//
//    let _ignored = remove_dir_all("sled");
//}

fn rkv() {
    extern crate rkv;
    use rkv::{Manager, Rkv, Store, Value};
    use std::path::Path;

    let _ignored = remove_dir_all("rkv");
    create_dir_all("rkv").unwrap();

    // sheesh!
    let created_arc = Manager::singleton()
        .write()
        .unwrap()
        .get_or_create(Path::new("rkv"), Rkv::new)
        .unwrap();
    let env = created_arc.read().unwrap();

    // Call `Rkv.open_or_create_default()` to get a handle to the default
    // (unnamed) store for the environment.
    let store: Store = env.open_or_create_default().unwrap();

    let data = [0u8; 1024 * 64];
    for i in 0..INIT {
        let mut key = [0u8; 16];
        BigEndian::write_u64(&mut key, i);
        let mut writer = env.write().unwrap();

        writer
            .put(store, &key, &Value::Blob(&data))
            .expect(&format!("writing {} failed", i));
        writer.commit().unwrap();
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
            let mut writer = env.write().unwrap();
            writer
                .put(store, &key, &Value::Blob(&data))
                .expect(&format!("writing {} failed", i));
            writer.commit().unwrap();
        }
    }

    println!(
        "sled validator-style writes/sec {}",
        per_second(WRITES, &now.elapsed())
    );

    // append-style
    let now = Instant::now();
    for i in 0..WRITES {
        let mut key = [0u8; 16];
        BigEndian::write_u64(&mut key, i + INIT + WRITES);
        let mut writer = env.write().unwrap();
        writer
            .put(store, &key, &Value::Blob(&data))
            .expect(&format!("writing {} failed", i));
        writer.commit().unwrap();
    }
    println!(
        "sled broadcast-style writes/sec {}",
        per_second(WRITES, &now.elapsed())
    );

    let _ignored = remove_dir_all("rkv");
}

fn main() {
    rocksdb();
    //    sled();
    rkv();
}
