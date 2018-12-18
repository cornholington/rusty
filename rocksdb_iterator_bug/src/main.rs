extern crate rocksdb;
use rocksdb::{Options, DB};

fn main() {
    let _ignored = DB::destroy(&Options::default(), "test_db");

    let mut iter = {
        let mut options = Options::default();
        options.create_if_missing(true);

        let db = DB::open(&options, "test_db").unwrap();

        for i in 0..100 {
            db.put(&[i], &[i]).expect(&format!("writing {} failed", i));
        }
        db.raw_iterator()
    };

    // iterate
    iter.seek_to_first();
    while iter.valid() {
        println!("Saw {:?} {:?}", iter.key(), iter.value());
        iter.next();
    }
    // crashes at some point
}
