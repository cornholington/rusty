fn main() {
    let mut loopvar = vec![1, 2, 3, 4];

    for _ in 0..loopvar.len() {
        println!("loopvar.pop() = {}", loopvar.pop().unwrap());
    }
}
