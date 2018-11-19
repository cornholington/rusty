use std::borrow::Cow;

fn main() {
    let x = Vec::new();

    x.extend(&[1, -2, 3]);

    let y = Cow::from(x);

    fn pop(input: &mut Cow<Vec<i32>>) -> () {
        input.to_mut().pop();
    }
    pop(&mut y);

    println!("x={:?}, y={:?}", x, y);
}
