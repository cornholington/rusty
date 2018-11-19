use std::fs::File;
use std::io::prelude::*;

extern crate bincode;
extern crate getopts;

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
    println!("In file {}", path.as_str());

    let mut f = File::open(path.as_str()).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}
