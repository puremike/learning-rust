#[allow(unused_assignments)]

pub fn example_expression() {
    let x: u32 = 5;

    let y: u32 = {
        let x_squared: u32 = x * x;
        let x_cube: u32 = x_squared * x;

        // This expression (without a semicolon) will be assigned to `y`
        x_cube + x_squared + x
    };

    let z: () = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        let _ = 2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

// Make it work with two ways
// fn main() {
//     let v = {
//         let mut x = 1;
//         x += 2
//     };

//     assert_eq!(v, 3);

//     println!("Success!");
//  }

// Make it work with two ways - First Way
fn exp1() {
    let v: i32 = {
        let mut x: i32 = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);

    println!("Exp 1 -> Success!");
}

// Make it work with two ways - Second Way
fn exp11() {
    let v: () = {
        let mut _x: i32 = 1;
        _x += 2; // Will return a Unit Type due to the semicolon at the end of the expression
    };

    assert_eq!(v, ());

    println!("Exp 1.1 -> Success!");
}

// fn main() {
//     let v = (let x = 3);

//     assert!(v == 3);

//     println!("Success!");
//  }

fn exp2() {
    let v: i32 = {
        let x: i32 = 3;
        x
    };

    assert!(v == 3);

    println!("Exp 2 -> Success!");
}

// fn main() {
//     let s = sum(1 , 2);
//     assert_eq!(s, 3);

//     println!("Success!");
// }

// fn sum(x: i32, y: i32) -> i32 {
//     x + y
// }

fn exp3() {
    let s: i32 = sum(1, 2);
    assert_eq!(s, 3);

    println!("Exp 3 -> Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
pub fn use_exp() {
    exp1();
    exp11();
    exp2();
    exp3();
}
