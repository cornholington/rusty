extern crate bincode;
extern crate inner;
use bincode::serialize;
use inner::inner::inner;

fn main() {
    println!("Hello, world! {:?}", serialize(&inner()));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
