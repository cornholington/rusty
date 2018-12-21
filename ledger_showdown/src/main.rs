extern crate byteorder;
use byteorder::{BigEndian, ByteOrder};
use rand::prelude::*;
//use std::sync::Arc;
//use std::thread;
use std::time::{Duration, Instant};

fn per_second(x: u64, elapsed: &Duration) -> f64 {
    let nanos = elapsed.as_secs() * 1_000_000_000 + elapsed.subsec_nanos() as u64;
    (x * 1_000_000_000) as f64 / nanos as f64
}
const INIT: u64 = 10_000;
const BLOCK: u64 = 100;
const WRITES: u64 = 10_000;

fn write_test<W>(name: &str, mut write: W)
where
    W: FnMut(&[u8], &[u8]) -> (),
{
    let mut data = [0u8; 1024 * 64];
    let mut key = [0u8; 16];

    let now = Instant::now();
    for i in 0..INIT {
        BigEndian::write_u64(&mut key, i);
        BigEndian::write_u64(&mut data, i);
        write(&key, &data);
        if i % 64 == 0 {
            print!("{} initializing {}...\r", name, i);
        }
    }
    println!(
        "{} initializing done at {} writes/sec",
        name,
        per_second(INIT, &now.elapsed())
    );

    let mut rng = rand::thread_rng();
    // random order for each chunk of a BLOCK
    let now = Instant::now();
    for i in 0..WRITES / BLOCK {
        let mut keys: Vec<u64> = (i..i + BLOCK).collect();
        keys.shuffle(&mut rng);
        for i in keys {
            BigEndian::write_u64(&mut key, i + INIT);
            BigEndian::write_u64(&mut data, i + INIT);
            write(&key, &data);
        }
    }

    println!(
        "{} validator-style {} writes/sec",
        name,
        per_second(WRITES, &now.elapsed())
    );

    let now = Instant::now();
    for i in 0..WRITES {
        BigEndian::write_u64(&mut key, i + INIT + WRITES);
        BigEndian::write_u64(&mut data, i + INIT + WRITES);

        write(&key, &data);
    }
    println!(
        "{} broadcast-style {} writes/sec",
        name,
        per_second(WRITES, &now.elapsed())
    );
}

pub fn rocksdb() {
    // rocksdb
    extern crate rocksdb;
    use rocksdb::{Options, DB};

    // cleanup
    let _ignored = DB::destroy(&Options::default(), "rocksdb");

    let mut options = Options::default();
    options.create_if_missing(true);

    let db = DB::open(&options, "rocksdb").expect("first open");

    write_test("rocksdb", |key, data| {
        db.put(key, data)
            .expect(&format!("writing {:?} failed", key));
    });

    // cleanup
    //    let _ignored = DB::destroy(&Options::default(), "rocksdb");
}

pub fn sled() {
    extern crate sled;
    use sled::{ConfigBuilder, Tree};
    use std::fs::remove_dir_all;

    let _ignored = remove_dir_all("sled");

    let config = ConfigBuilder::default()
        .path("sled".to_owned())
        .cache_capacity(10_000_000_000)
        .use_compression(true)
        .flush_every_ms(Some(1000))
        .snapshot_after_ops(100_000)
        .build();

    let tree = Tree::start(config).unwrap();

    write_test("sled", |key, data| {
        tree.set(key, data.to_vec())
            .expect(&format!("writing {:?} failed", key));
    });

    //    let _ignored = remove_dir_all("sled");
}

pub fn rkv() {
    extern crate rkv;
    use rkv::{Manager, Rkv, Store, Value};
    use std::fs::{create_dir_all, remove_dir_all};
    use std::path::Path;

    let _ignored = remove_dir_all("rkv");
    create_dir_all("rkv").unwrap();

    // sheesh!
    let created_arc = Manager::singleton()
        .write()
        .unwrap()
        .get_or_create_with_capacity(Path::new("rkv"), 1024 * 1024 * 1024, Rkv::with_capacity)
        .unwrap();
    let env = created_arc.read().unwrap();

    // Call `Rkv.open_or_create_default()` to get a handle to the default
    // (unnamed) store for the environment.
    let store: Store = env.open_or_create_default().unwrap();

    write_test("rkv", |key, data| {
        let mut writer = env.write().unwrap();
        writer
            .put(store, &key, &Value::Blob(&data))
            .expect(&format!("writing {:?} failed", key));
        writer.commit().unwrap();
    });

    //    let _ignored = remove_dir_all("rkv");
}

pub fn files() {
    use std::fs::{create_dir_all, remove_dir_all, write};
    use std::path::PathBuf;

    let _ignored = remove_dir_all("files");
    create_dir_all("files").unwrap();

    let hex_strs = [
        "00", "01", "02", "03", "04", "05", "06", "07", "08", "09", "0A", "0B", "0C", "0D", "0E",
        "0F", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "1A", "1B", "1C", "1D",
        "1E", "1F", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "2A", "2B", "2C",
        "2D", "2E", "2F", "30", "31", "32", "33", "34", "35", "36", "37", "38", "39", "3A", "3B",
        "3C", "3D", "3E", "3F", "40", "41", "42", "43", "44", "45", "46", "47", "48", "49", "4A",
        "4B", "4C", "4D", "4E", "4F", "50", "51", "52", "53", "54", "55", "56", "57", "58", "59",
        "5A", "5B", "5C", "5D", "5E", "5F", "60", "61", "62", "63", "64", "65", "66", "67", "68",
        "69", "6A", "6B", "6C", "6D", "6E", "6F", "70", "71", "72", "73", "74", "75", "76", "77",
        "78", "79", "7A", "7B", "7C", "7D", "7E", "7F", "80", "81", "82", "83", "84", "85", "86",
        "87", "88", "89", "8A", "8B", "8C", "8D", "8E", "8F", "90", "91", "92", "93", "94", "95",
        "96", "97", "98", "99", "9A", "9B", "9C", "9D", "9E", "9F", "A0", "A1", "A2", "A3", "A4",
        "A5", "A6", "A7", "A8", "A9", "AA", "AB", "AC", "AD", "AE", "AF", "B0", "B1", "B2", "B3",
        "B4", "B5", "B6", "B7", "B8", "B9", "BA", "BB", "BC", "BD", "BE", "BF", "C0", "C1", "C2",
        "C3", "C4", "C5", "C6", "C7", "C8", "C9", "CA", "CB", "CC", "CD", "CE", "CF", "D0", "D1",
        "D2", "D3", "D4", "D5", "D6", "D7", "D8", "D9", "DA", "DB", "DC", "DD", "DE", "DF", "E0",
        "E1", "E2", "E3", "E4", "E5", "E6", "E7", "E8", "E9", "EA", "EB", "EC", "ED", "EE", "EF",
        "F0", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "FA", "FB", "FC", "FD", "FE",
        "FF",
    ];

    write_test("files", |key, data| {
        let file: Vec<_> = key.iter().map(|b| hex_strs[*b as usize]).collect();
        let file = file.join("");
        let mut path = PathBuf::from("files");
        path.push(&file);
        write(path, data).expect(&format!("write failed"));
    });

    //    let _ignored = remove_dir_all("files");
}

pub fn two_files() {
    use std::fs::{create_dir_all, remove_dir_all, OpenOptions};
    use std::io::{BufWriter, Write};

    let _ignored = remove_dir_all("two_files");
    create_dir_all("two_files").unwrap();

    let keyfile = OpenOptions::new()
        .create(true)
        .append(true)
        .open("two_files/key")
        .unwrap();
    let mut keyfile = BufWriter::new(keyfile);

    let datafile = OpenOptions::new()
        .create(true)
        .append(true)
        .open("two_files/data")
        .unwrap();
    let mut datafile = BufWriter::new(datafile);

    write_test("two_files", |key, data| {
        keyfile
            .write(key)
            .expect(&format!("write key {:?} failed", key));
        datafile
            .write(data)
            .expect(&format!("write data {:?} failed", key));
    });

    //    let _ignored = remove_dir_all("file");
}

fn main() {
    println!("Hello World!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_files() {
        files();
    }
    #[test]
    fn test_two_files() {
        two_files();
    }
    #[test]
    fn test_sled() {
        sled();
    }
    #[test]
    fn test_rocksdb() {
        rocksdb();
    }
    #[test]
    fn test_rkv() {
        rkv();
    }

}
