// #[allow(unused_assignments)]

// // Remove something to make it work
// fn main() {
//     let x: i32 = 5;
//     let mut y: u32 = 5;

//     y = x;

//     let z = 10; // Type of z ?
// }

use std::any::type_name;
use std::ops::{Range, RangeInclusive};

fn nt1() {
    let x: i32 = 5;
    let mut y: u32 = 5;

    println!("nt1 -> y = {}", y + 10);

    y = x as u32;

    let _z: i32 = 10;

    println!("nt1 -> y = {}", y);
}

// Fill the blank
// fn main() {
//     let v: u16 = 38_u8 as __;

//     println!("Success!");
// }

// Fill the blank
fn nt2() {
    let v: u16 = 38_u8 as u16;

    println!("nt2 -> {}", v);
}

// Modify `assert_eq!` to make it work
// fn main() {
//     let x = 5;
//     assert_eq!("u32".to_string(), type_of(&x));

//     println!("Success!");
// }

// // Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }

fn nt3() {
    let x: i32 = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("nt4 -> Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", type_name::<T>())
}

// Fill the blanks to make it work
// fn main() {
//     assert_eq!(i8::MAX, __);
//     assert_eq!(u8::MAX, __);

//     println!("Success!");
// }

fn nt4() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("nt4 -> Success!");
}

// Fix errors and panics to make it work
// fn main() {
//    let v1 = 251_u8 + 8;
//    let v2 = i8::checked_add(251, 8).unwrap();
//    println!("{},{}",v1,v2);
// }

fn nt5() {
    let v1: u8 = 251_u8 + 3;
    let v2: i8 = i8::checked_add(107, 8).unwrap();
    println!("{},{}", v1, v2);
}

// Modify `assert!` to make it work
// fn main() {
//     let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
//     assert!(v == 1579);

//     println!("Success!");
// }

// Modify `assert!` to make it work
fn nt6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v != 1579);

    println!("nt6 -> Success!");
}

// Fill the blank to make it work
// fn main() {
//     let x = 1_000.000_1; // ?
//     let y: f32 = 0.12; // f32
//     let z = 0.01_f64; // f64

//     assert_eq!(type_of(&x), "__".to_string());
//     println!("Success!");
// }

// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }

fn nt7() {
    let x = 1_000.000_1; // ?
    let _y: f32 = 0.12; // f32
    let _z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("nt7 -> Success!");
}

// fn main() {
//     assert!(0.1+0.2==0.3);

//     println!("Success!");
// }

fn nt8() {
    let sum: f64 = 0.1 + 0.2;
    let expected: f64 = 0.3;
    let eps: f64 = f64::EPSILON;

    // let eps1: f64 = 1e-10;
    assert!((sum - expected).abs() < eps);

    println!("nt8 -> True");
}

fn nt81() {
    assert!(0.1f32 + 0.2f32 == 0.3f32);
    println!("nt81 -> True");
}

// fn main() {
//     let mut sum = 0;
//     for i in -3..2 {
//         sum += i
//     }

//     assert!(sum == -3);

//     for c in 'a'..='z' {
//         println!("{}",c);
//     }
// }

// In Rust, the syntax -3..2 represents a range from -3 up to, but not including, 2. This is called a half-open range (start..end) --> -3, -2, -1, 0, 1
fn nt9() {
    // let sum: i32 = (-3..2).sum();

    let mut sum: i32 = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum != -3);

    // prints number between a to z (z inclusive)
    for c in 'a'..='e' {
        println!("{}", c);
    }

    // prints number between 97 to 123 (123 inclusive)
    for n in 97..=102 {
        println!("{}", n);
    }

    // still the same as 1..=10
    for m in RangeInclusive::new(1, 7) {
        println!("{}", m);
    }

    println!("nt9 -> Success!");
}

// Fill the blanks
// use std::ops::{Range, RangeInclusive};
// fn main() {
//     assert_eq!((1..__), Range{ start: 1, end: 5 });
//     assert_eq!((1..__), RangeInclusive::new(1, 5));

//     println!("Success!");
// }

// Fill the blanks

fn nt10() {
    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("nt10 -> Success!");
}

// Fill the blanks and fix the errors
// fn main() {
//     // Integer addition
//     assert!(1u32 + 2 == __);

//     // Integer subtraction
//     assert!(1i32 - 2 == __);
//     assert!(1u8 - 2 == -1);

//     assert!(3 * 50 == __);

//     assert!(9.6 / 3.2 == 3.0); // error ! make it work

//     assert!(24 % 5 == __);
//     // Short-circuiting boolean logic
//     assert!(true && false == __);
//     assert!(true || false == __);
//     assert!(!true == __);

//     // Bitwise operations
//     println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
//     println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
//     println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
//     println!("1 << 5 is {}", 1u32 << 5);
//     println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
// }

fn nt11() {
    // Integer addition
    assert!(1u32 + 2u32 == 3u32);

    // Integer subtraction
    assert!(1i32 - 2i32 == -1i32);
    assert!(1i8 - 2i8 == -1i8);

    assert!(3 * 50 == 150);

    assert!(9.6f32 / 3.2f32 == 3f32);

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
pub fn usenumbertypes() {
    nt1();
    nt2();
    nt3();
    nt4();
    nt5();
    nt6();
    nt7();
    nt8();
    nt81();
    nt9();
    nt10();
    nt11();
}
