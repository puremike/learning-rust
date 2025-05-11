// Ownership
// • Rust's ownership system is unique and sets it apart from other programming languages
// • Set of rules that govern memory management
// • Rules are enforced at compile time
// • If any of the rules are violated, the program won't compile

// Three Rules of Ownership
// 1. Each value in Rust has an owner
// 2. There can only be one owner at a time
// 3. When the owner goes out of scope, the value will be dropped

// **Owner: The owner of a value is the variable or data structure that holds it and is responsible for allocating and freeing the memory used to store that data

// Memory
// • Component in a computer to store data and instructions for the processor to execute
// • Random Access Memory (RAM) is volatile, when power turned off all contents are lost.
// • Two types of regions in RAM used by a program at runtime: Stack Memory and Heap Memory

// Stack Memory
// • Last in, first out
// • All data stored on the stack must have a known, fixed size (like integers, floats, char, bool, etc...)
// • Pushing to the stack is faster than allocating on the heap, because the location for new data is always at the top of the stack
// • Types of unknown size will get allocated to the heap and a pointer to the value is pushed to the stack, because a pointer is fixed size (usize)

// Heap Memory
// • Data of no known, fixed size belongs on the heap
// • Allocating data on the heap will return a pointer (an address to location where data has been allocated)
// • Allocating on the heap is slower than pushing to stack
// • Accessing data on the heap is also slower, as it has to be accessed using a pointer which points to an address. This is due to - pointer indirection (an extra memory lookup), and possible cache misses and less predictable memory locality.

// An example of the Heap Allocated type is the String Type.
// The String Type
// • All types covered so far were fixed size
// • String is mutable
// • String size can change at runtime
// • String stored on the stack with a pointer to the heap
// • Value of String is stored on the heap

fn stack_heap() {
    // So, the first variable x will be dropped and cannot be used after assigning it to y, to avoid dangling pointers
    let x: String = String::from("Hello World!");
    // let y: String = x;

    // To ensure that data between variable of String type is not dropped for another variable, you will have to DEEP COPY it
    let y: String = x.to_uppercase().clone();

    println!("y = {}, x = {}", y, x); // y is "HELLO WORLD!" and x is "Hello World!"
}

fn checking_ownership() {
    let _s1: String = gives_ownership(); // gives ownership moves its return value into s1

    let s2: String = String::from("hello_s2"); // s2 comes into scope

    let _s3: String = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3
}

// Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens to s2. s1 goes out of scope and is dropped

fn gives_ownership() -> String {
    let some_string: String = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// fn main() {
//     // Use as many approaches as you can to make it work
//     let x = String::from("Hello world");
//     let y = x;
//     println!("{}, {}",x, y);
// }

fn own1() {
    let x: String = String::from("Hello world");
    let y: String = x.clone();
    println!("Own 1 -> x = {}, y = {}", x, y);
}

// Don't modify code in main!
// fn main() {
//     let s1 = String::from("Hello world");
//     let s2 = take_ownership(s1);

//     println!("{}", s2);
// }

// Don't modify code in own2!
fn own2() {
    let s1: String = String::from("Hello world");
    let s2: String = take_ownership(s1);

    println!("Own 2 -> {}", s2);
}
// Only modify the code below!
fn take_ownership(s: String) -> String {
    println!("Take ownership -> {}", s);
    s
}

// fn main() {
//     let s = give_ownership();
//     println!("{}", s);
// }

// // Only modify the code below!
// fn give_ownership() -> String {
//     let s = String::from("Hello world");
//     // Convert String to Vec
//     let _s = s.into_bytes();
//     s
// }

fn own3() {
    let s: String = give_ownership();
    println!("Own3 -> {}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s: String = String::from("Hello world");
    // Convert String to Vec
    let y: &[u8] = s.as_bytes();
    println!("Into Vec<u8> -> {:?}", y);
    s
}

// Fix the error without removing any code
// fn main() {
//     let s = String::from("Hello World");

//     print_str(s);

//     println!("{}", s);
// }

// fn print_str(s: String) {
//     println!("{}", s)
// }

fn own4() {
    let s: String = String::from("Hello World");

    print_str(s.clone());

    println!("Own4 -> {}", s);
}

fn print_str(s: String) {
    println!("{}", s)
}

// Don't use clone ,use copy instead
// fn main() {
//     let x = (1, 2, (), "hello".to_string());
//     let y = x.clone();
//     println!("{:?}, {:?}", x, y);
// }

// Don't use clone ,use copy instead
fn own5() {
    // let x: (i32, i32, (), String) = (1, 2, (), "hello".to_string());
    let x: (i32, i32, (), &str) = (1, 2, (), "hello");
    let y: (i32, i32, (), &str) = x; // removing String from the tuple will and using a borrowed &str will automatically copy x into y. You can't copy a String (it's HEAP allocateds)
    println!("{:?}, {:?}", x, y);
}

// make the necessary variable mutabled
// fn main() {
//     let s = String::from("Hello ");

//     let s1 = s;

//     s1.push_str("World!");

//     println!("Success!");
// }

// make the necessary variable mutable
fn own6() {
    let s: String = String::from("Hello ");

    let mut s1: String = s;

    s1.push_str("World!");

    println!("Own 6 -> Success!");
}

// fn main() {
//     let x = Box::new(5);

//     let ...      // update this line, don't change other lines!

//     *y = 4;

//     assert_eq!(*x, 5);

//     println!("Success!");
// }

fn own7() {
    let x: Box<i32> = Box::new(5); // A Box<T> is a smart pointer using the HEAP allocation

    let mut y: Box<i32> = x.clone(); // update this line, don't change other lines!

    *y = 4;

    assert_eq!(*x, 5);

    println!("Own 7 -> Success!");
}

// fn main() {
//     let t = (String::from("hello"), String::from("world"));

//     let _s = t.0;

//     // Modify this line only, don't use `_s`
//     println!("{:?}", t);
//  }

fn own8() {
    let t: (String, String) = (String::from("hello"), String::from("world"));

    let _s: String = t.0;

    // Modify this line only, don't use `_s`
    println!("Own 8 -> {:?}", t.1);
}

// fn main() {
//     let t = (String::from("hello"), String::from("world"));

//      // Fill the blanks
//      let (__, __) = __;

//      println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
//  }

fn own9() {
    let t: (String, String) = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (s1, s2): (String, String) = t.clone();

    println!("Own 9 -> {:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}

pub fn use_stack_heap() {
    stack_heap();
    checking_ownership();
    own1();
    own2();
    own3();
    own4();
    own5();
    own6();
    own7();
    own8();
    own9();
}
