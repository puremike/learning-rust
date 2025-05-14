// String vs. &str
// • A String is a heap-allocated string type that owns its contents and is mutable
// • Abstr is an immutable sequence of UTF-8 bytes in memory, it does not own the underlying data and is immutable
// • Think of &str as a view on a sequence of characters (stored as UTF-8 bytes) in memory

// String vs. &str
// • Use &str if you just want to a view ofa string
// • &str is more lightweight and efficient than String
// • Use String if you need to own the data and be able to mutate it

// Fix error without adding new line
// fn main() {
//     let s: str = "hello, world";

//     println!("Success!");
// }

fn sstr1() {
    let _s: &str = "hello, world";

    println!("Sstr 1 -> Success!");
}

// Fix the error with at least two solutions
// fn main() {
//     let s: Box<str> = "hello, world".into();
//     greetings(s)
// }

fn greetings(s: &str) {
    println!("Sstr 2 -> {}", s)
}

fn sstr2() {
    let s: &str = "hello, world".into();
    greetings(s)
}

// Fill the blank
// fn main() {
//     let mut s = __;
//     s.push_str("hello, world");
//     s.push('!');

//     assert_eq!(s, "hello, world!");

//     println!("Success!");
// }

fn sstr3() {
    let mut s: String = String::new();
    s.push_str("hello, world");
    s.push('!');
    assert_eq!(s, "hello, world!");
    println!("Sstr 3 -> Success!");
}

// Fill the blank
// fn main() {
//     let s = String::from("I like dogs");
//     // Allocate new memory and store the modified string there
//     let s1 = s.__("dogs", "cats");

//     assert_eq!(s1, "I like cats");

//     println!("Success!");
// }

// Fill the blank
fn sstr4() {
    let s: String = String::from("I like dogs");
    let s1: String = s.replace("dogs", "cats");
    assert_eq!(s1, "I like cats");
    println!("Success!");
}

// Fix errors without removing any line
// fn main() {
//     let s1 = String::from("hello,");
//     let s2 = String::from("world!");
//     let s3 = s1 + s2;
//     assert_eq!(s3, "hello,world!");
//     println!("{}", s1);
// }

// Fix errors without removing any line
fn sstr5() {
    let s1: String = String::from("hello,");
    let s2: String = String::from("world!");
    let s3: String = s1.clone() + &s2;
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);
}

pub fn use_stringvsstr() {
    sstr1();
    sstr2();
    sstr3();
    sstr4();
    sstr5();
}
