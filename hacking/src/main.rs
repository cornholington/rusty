//fn five() -> f64 {
//    5.0
//    // doesn't work
//    //  5
//}

fn main() {
    println!("2/3 = {}", 2/3);

    println!("2.0/3.0 = {}", 2.0/3.0);

    // doesn't work
    //  println!("2/3.0 = {}", 2/3.0);

    // doesn't work
    //   println!("( 2.0, 3.0 ) = {}", (2.0, 3.0));

    let x = five();
    println!("The value of x is: {}", x);

    // this five() overrides outer five() even for code above it,
    //  results in a compiler warning
    fn five() -> isize {
        5
    }

    let x = five();
    println!("The value of x is: {}", x);

    // TODO: figure out what while {} returns...
    //    let mut i = 5;
    //    let mut j = 0;
    //    let x = while i > 0 {
    //        i = i - 1;
    //        ( j = j + 1 )
    //    };
    //  doesn't work...
    //    println!("The value of i is: {}, j is {}, x is {}",
    //       i, j, x.0);

    // Strings are structs on the stack, has the "Drop" trait
    // here's a move that fails
    // let s1 = String::from("hello");
    // let s2 = s1; // move!
    //
    // println!("{}, world!", s1);

    // s is a struct on the stack, has the "Drop" trait
    let s = String::from("hello");

    // pass a String by value
    fn takes_ownership(some_string: String) {
        println!("{}", some_string);
    }

    fn gives_ownership() -> String {             // gives_ownership will move its
        // return value into the function
        // that calls it

        let some_string = String::from("hello"); // some_string comes into scope

        some_string                              // some_string is returned and
        // moves out to the calling
        // function.
    }

    // shit gets weird
    takes_ownership(gives_ownership());

    fn borrows_or_does_it(s: &String) {
        takes_ownership(s.to_string());
        // can't pass just s, get an error
        //  s.to_string() must create a new String...
    }

    borrows_or_does_it(&s);
    takes_ownership(s);
    // is an error...
    // println!("{}", s);

    // this bit doesn't work, can't smoosh s while there's an immutable ref
    // let mut s = String::from("hello");
    // let y = &s;
    // s = String::from("bye");

//    let mut s = String::from("hello");
//
//    println!("{}", s);
//
//    let slicex = &s[..3];
//
//    println!("{}", slicex);
//
//    let slicey = &s[0..2];
//
//    println!("{}", slicey);
//
//    let mut s = String::from("hello");


    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]); // this is ODD... &str is the type
    // equivalently        my_string.as_str()??

    println!("{}", word);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);
    println!("{}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    println!("{}", word);

    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let vec: Vec<u32> = vec![];
    for chunk in vec.chunks(1) {
        println!("empty vector has a chunk");
    }
}
