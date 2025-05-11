// fn main() {
//     // Don't modify the following two lines!
//     let (x, y) = (1, 2);
//     let s = sum(x, y);

//     assert_eq!(s, 3);

//     println!("Success!");
// }

// fn sum(x, y: i32) {
//     x + y;
// }

fn func1() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s: () = sum(x, y);

    assert_eq!(s, ());

    println!("Fun 1 -> Success!");
}

fn sum(x: i32, y: i32) {
    let _ = x + y; // this will return a Unit Type
}

// fn main() {
//     print();
//  }

//  // Replace i32 with another type
//  fn print() -> i32 {
//     println!("Success!");
//  }

fn func2() {
    print();
}

// Replace i32 with another type
fn print() {
    println!("Success!"); // will return a unit type
}

// Solve it in two ways
// DON'T let `println!` work
// fn main() {
//     never_return();

//     println!("Failed!");
// }

// fn never_return() -> ! {
//     // Implement this function, don't modify the fn signatures

// }

// fn func3() {
//     never_return();

//     println!("Failed!");
// }

// fn never_return() -> ! {
//     // Implement this function, don't modify the fn signatures
//     panic!();
// }

// fn main() {
//     println!("Success!");
// }

// fn get_option(tp: u8) -> Option<i32> {
//     match tp {
//         1 => {
//             // TODO
//         }
//         _ => {
//             // TODO
//         }
//     };

//     // Rather than returning a None, we use a diverging function instead
//     never_return_fn()
// }

// // IMPLEMENT this function in THREE ways
// fn never_return_fn() -> ! {

// }

fn func4() {
    println!("Func 4 -> Success!");
    get_option(1);
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => Option::from(144),
        _ => Option::from(244),
    }

    // Rather than returning a None, we use a diverging function instead
    // never_return_fn()
}

// // IMPLEMENT this function in THREE ways
// fn never_return_fn() -> ! {
//     panic!();
//     // todo!();
//     // unimplemented!();
// }

// fn main() {
//     // FILL in the blank
//     let b = __;

//     let _v = match b {
//         true => 1,
//         // Diverging functions can also be used in match expression to replace a value of any value
//         false => {
//             println!("Success!");
//             panic!("we have no value for `false`, but we can panic");
//         }
//     };

//     println!("Exercise Failed if printing out this line!");
// }

fn func5() {
    // FILL in the blank
    let b: bool = false;

    let _v: i32 = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Func 5 -> Success!");

            // returned a value of 10 because I don't want to panic
            let x: i32 = 10;
            x
            // panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}

pub fn use_func() {
    func1();
    func2();
    // func3();
    func4();
    func5();
}
