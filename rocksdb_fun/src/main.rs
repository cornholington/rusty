extern crate byteorder;
extern crate rocksdb;
use byteorder::{BigEndian, ByteOrder};
use rocksdb::{Options, DB};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let _ignored = DB::destroy(&Options::default(), "test_db");

    let mut options = Options::default();
    options.create_if_missing(true);

    let db = Arc::new(DB::open(&options, "test_db").expect("first open"));

    let _reader = {
        let db = db.clone();
        thread::spawn(move || loop {
            let mut iter = db.raw_iterator();
            // iterate
            iter.seek_to_first();
            while iter.valid() {
                //                println!("thread {:?} {:?}", iter.key(), iter.value());
                iter.next();
            }
        })
    };

    let _writer = {
        let db = db.clone();
        thread::spawn(move || {
            let mut writes = 0f64;
            let mut now = Instant::now();
            let one_s = Duration::from_millis(1_000);

            let mut i = 0;
            loop {
                let mut buf = [0u8; 8];
                BigEndian::write_u64(&mut buf, i);
                db.put(&buf, &buf).expect(&format!("writing {} failed", i));
                i += 1;
                writes += 1f64;
                let elapsed = now.elapsed();
                if elapsed > one_s {
                    let elapsed = (elapsed.as_secs() * 1_000_000_000
                        + elapsed.subsec_nanos() as u64) as f64
                        / 1_000_000_000f64;
                    println!("{} entries, {} writes/sec", i, writes / elapsed);
                    now = Instant::now();
                    writes = 0f64;
                }
            }
        })
    };

    let mut reads = 0f64;
    let mut now = Instant::now();
    let one_s = Duration::from_millis(1_000);

    loop {
        let mut iter = db.raw_iterator();

        iter.seek_to_first();
        while iter.valid() {
            iter.next();
            reads += 1f64;
            let elapsed = now.elapsed();
            if elapsed > one_s {
                let elapsed = (elapsed.as_secs() * 1_000_000_000 + elapsed.subsec_nanos() as u64)
                    as f64
                    / 1_000_000_000f64;
                println!("{} reads/sec", reads / elapsed);
                now = Instant::now();
                reads = 0f64;
            }
        }
    }
    //    let _ = thread.join();
}
