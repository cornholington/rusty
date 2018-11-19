fn main() {
    let mut foo = [4, 5, 6].to_vec();

    foo.insert(0, 3);
    foo.insert(0, 2);
    foo.insert(0, 1);

    println!("{:?}", foo);
}
