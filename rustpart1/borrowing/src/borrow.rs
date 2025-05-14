// Borrowing
// Way of temporarily access data without taking ownership ot it
// • When borrowing, you're taking a reference (pointer) to the data, not the data itself
// • Prevention of dangling pointers and data races
// • Data can be borrowed immutabily and mutably
// • There are certain rules when borrowing which we have to comply with, otherwise the program won't compile

// Rules of References
// 1. At any given time, you can have either one mutable reference OR any number of immutable references
// 2.References must always be valid

fn borrow1() {
    let s: String = String::from("HW!");
    let s1: &String = &s;

    println!("Borrow 1 -> {}, {}", s, s1);
}

fn borrow2() {
    let s: String = String::from("HW!");
    let s1: &String = &s;
    let s2: &String = &s;

    // satisfies the condition of rules of references --> we can have multiple immutable references

    println!("Borrow 2 -> {}, {}, {}", s, s1, s2);
}

fn borrow3() {
    let mut s: String = String::from("HAAA!");
    let s2: &mut String = &mut s;
    // let s3: &mut String = &mut s; // adding this line doesn't satisfy the rule --> only can mutable reference can be made

    println!("Borrow 3 -> {}", s2);
}

// fn main() {
//     let x = 5;
//     // Fill the blank
//     let p = __;

//     println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
//  }

fn borrow4() {
    let x: i32 = 5;
    let p: &i32 = &x;

    println!("The memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
    println!("Borrow 4 -> The value of p is {}", *p); // or just p because {} deferences a pointer
}

// fn main() {
//     let x = 5;
//     let y = &x;

//     // Modify this line only
//     assert_eq!(5, y);

//     println!("Success!");

fn borrow5() {
    let x: i32 = 5;
    let y: &i32 = &x;

    // *y will automatically deference the pointer
    assert_eq!(5, *y);

    println!("Borrow 5 -> Success!");
}

// Fix error
// fn main() {
//     let mut s = String::from("hello, ");

//     borrow_object(s);

//     println!("Success!");
// }

fn borrow_object(_s: &String) {}

// Fix error
fn borrow6() {
    let s: String = String::from("hello, ");
    borrow_object(&s);
    println!("Borrow 6 -> Success!");
}

// fn main() {
//     let mut s = String::from("hello, ");

//     push_str(s);

//     println!("Success!");
// }

fn push_str(s: &mut String) {
    s.push_str("world")
}

fn borrow7() {
    let mut s: String = String::from("hello, ");
    push_str(&mut s);
    println!("Borrow 7 -> Success!");
}

fn borrow8() {
    let mut s = String::from("hello, ");
    // Fill the blank to make it work
    let p: &mut String = &mut s;
    p.push_str("world");
    println!("Borrow 8 -> Success!");
}

// fn main() {
//     let c = '中';

//     let r1 = &c;
//     // Fill the blank，dont change other code
//     let __ r2 = c;

//     assert_eq!(*r1, *r2);

//     // Check the equality of the two address strings
//     assert_eq!(get_addr(r1),get_addr(r2));

//     println!("Success!");
// }

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

fn borrow9() {
    let c: char = '中';

    let r1: &char = &c;
    let ref r2: char = c;

    assert_eq!(*r1, *r2);

    // Check the equality of the two address strings
    assert_eq!(get_addr(r1), get_addr(r2));

    println!("Borrow 9 -> Success!");
}

// Remove something to make it work
// Don't remove a whole line !
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &mut s;

//     println!("{}, {}", r1, r2);

//     println!("Success!");
// }

fn borrow10() {
    let s: String = String::from("hello");
    let r1: &String = &s;
    let r2: &String = &s;

    println!("{}, {}", r1, r2);

    println!("Success!");
}

// This code has no errors!
// fn main() {
//     let mut s = String::from("hello, ");

//     borrow_object(&s);

//     s.push_str("world");

//     println!("Success!");
// }

fn borrow11() {
    let mut s: String = String::from("hello, ");

    borrow_object1(&s);

    s.push_str("world");

    println!("Success!");
}

fn borrow_object1(_s: &String) {}

pub fn use_borrow() {
    borrow1();
    borrow2();
    borrow3();
    borrow4();
    borrow5();
    borrow6();
    borrow7();
    borrow8();
    borrow9();
    borrow10();
    borrow11();
}
