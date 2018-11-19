fn main() {
    let lines: Vec<_> = { 0..10 }.map(|x| format!("{}\n", x)).collect();

    print!("{}", lines.join(""));
}
