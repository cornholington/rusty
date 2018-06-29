extern crate bincode;
extern crate getopts;
extern crate leveldb;

use leveldb::database::Database;
use leveldb::iterator::Iterable;
use leveldb::kv::KV;
use leveldb::options::{Options, ReadOptions, WriteOptions};

fn usage(opts: getopts::Options) {
    let brief = format!("Usage: {} [options]\n", std::env::args().next().unwrap());
    print!("{}\n\n", opts.usage(&brief));
}

fn main() {
    let mut opts = getopts::Options::new();
    opts.optflag("h", "help", "print help");
    opts.optopt("j", "journal", "use journal directory DIR", "DIR");

    let args: Vec<String> = std::env::args().collect();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    if matches.opt_present("h") {
        usage(opts);
        std::process::exit(0);
    }

    let path = if matches.opt_present("j") {
        matches.opt_str("j").unwrap()
    } else {
        usage(opts);
        std::process::exit(0);
    };

    let path = std::path::Path::new(&path);

    let mut options = Options::new();
    options.create_if_missing = true;
    let database = match Database::open(path, options) {
        Ok(db) => db,
        Err(e) => panic!("failed to open database: {:?}", e),
    };

    let write_opts = WriteOptions::new();
    for i in 0..1000000 {
        let val = bincode::serialize(&i).unwrap();
        match database.put(write_opts, i, &val) {
            Ok(_) => (),
            Err(e) => panic!("failed to write to database: {:?}", e),
        };

        let read_opts = ReadOptions::new();
        let res = database.get(read_opts, i);

        match res {
            Ok(data) => {
                assert!(data.is_some());
                let di: i32 = bincode::deserialize(&data.unwrap()).unwrap();
                assert_eq!(di, i);
            }
            Err(e) => panic!("failed reading data: {:?}", e),
        }
    }

    let read_opts = ReadOptions::new();
    let iter = database.iter(read_opts);
    let mut i = 0;
    for entry in iter {
        let val = bincode::serialize(&i).unwrap();
        assert_eq!(entry, (i, val));
        i += 1;
    }
}
