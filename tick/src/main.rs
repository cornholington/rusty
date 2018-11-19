use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

fn main() {
    let timeout = Duration::new(std::u64::MAX, 0); // forever
    let last_tick = Instant::now();

    let timeout_left = timeout - (Instant::now() - last_tick);
    println!(
        "dur={:?} now-last_tick={:?}",
        timeout,
        Instant::now() - last_tick,
    );

    println!("timeout_left {:?}", timeout_left);
}
