use std::sync::mpsc::channel;

fn main() {
    let (sender, _receiver) = channel();

    let mut i: u64 = 0;
    loop {
        sender.send(i).unwrap();
        i += 1;
    }
}
