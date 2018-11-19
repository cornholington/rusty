fn main() {
    let mut tail = vec![10, 11, 12, 13, 4, 5, 6, 7, 8, 9];
    let mut tail_idx = 4;
    let mut i = 0;
    println!("{:?}", tail);
    while i != tail_idx {
        tail.swap(i, tail_idx);
        println!("{:?} {} {}", tail, i, tail_idx);

        i += 1;
        tail_idx += 1;
        if tail_idx == tail.len() {
            tail_idx = tail.len() - 1;
        }
    }
}
