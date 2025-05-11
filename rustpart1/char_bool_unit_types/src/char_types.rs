// Char has a size of 4 bytes
// Bool has a sixe of 1 byte
// Unit has a size of 0

// // Make it work
// use std::mem::size_of_val;
// fn main() {
//     let c1 = 'a';
//     assert_eq!(size_of_val(&c1),1);

//     let c2 = '中';
//     assert_eq!(size_of_val(&c2),3);

//     println!("Success!");
// }

// Make it work
fn char1() {
    let c1: char = 'a';
    // println!("{}", size_of_val(&c1));
    assert_eq!(size_of_val(&c1), 4);

    let c2: char = '中';
    // println!("{}", size_of_val(&c2));
    assert_eq!(size_of_val(&c2), 4);

    println!("Char 1 -> Success!");
}

// // Make it work
// fn main() {
//     let c1 = "中";
//     print_char(c1);
// }

// fn print_char(c : char) {
//     println!("{}", c);
// }

fn char2() {
    let c1: char = '中';
    print_char(c1);
}

fn print_char(c: char) {
    println!("{}", c);
}

// Make println! work
// fn main() {
//     let _f: bool = false;

//     let t = true;
//     if !t {
//         println!("Success!");
//     }
// }

// Make println! work
fn char3() {
    let _f: bool = false;

    let t: bool = true;
    if t {
        println!("Char 3 -> Success!");
    }
}

// / Make it work
// fn main() {
//     let f = true;
//     let t = true && false;
//     assert_eq!(t, f);

//     println!("Success!");
// }

fn char4() {
    let f: bool = true;
    let t: bool = true || false;
    assert_eq!(t, f);

    println!("Char4 -> Success!");
}

// Make it work, don't modify `implicitly_ret_unit` !
// fn main() {
//     let _v: () = ();

//     let v = (2, 3);
//     assert_eq!(v, implicitly_ret_unit());

//     println!("Success!");
// }

// fn implicitly_ret_unit() {
//     println!("I will return a ()");
// }

// // Don't use this one
// fn explicitly_ret_unit() -> () {
//     println!("I will return a ()");
// }

// Make it work, don't modify `implicitly_ret_unit` !
fn char5() {
    let _v: () = ();

    let v: (i32, i32) = (2, 3);
    println!("v = ({}, {})", v.0, v.1);

    assert_eq!(_v, implicitly_ret_unit());

    println!("Char 5 -> Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// // Don't use this one
// fn explicitly_ret_unit() -> () {
//     println!("I will return a ()");
// }

// Modify `4` in assert to make it work
// use std::mem::size_of_val;
// fn main() {
//     let unit: () = ();
//     assert!(size_of_val(&unit) == 4);

//     println!("Success!");
// }

// Modify `4` in assert to make it work

fn char6() {
    let unit: () = ();
    // println!("unit size = {}", size_of_val(&unit));
    assert!(size_of_val(&unit) == 0);

    println!("Char 6 -> Success!");
}

pub fn use_char() {
    char1();
    char2();
    char3();
    char4();
    char5();
    char6();
}
