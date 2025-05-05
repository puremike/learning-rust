// Fix the error below with least amount of modification to the code
// fn main() {
//     let x: i32; // Uninitialized but used, ERROR !
//     let y: i32; // Uninitialized but also unused, only a Warning !

//     assert_eq!(x, 5);
//     println!("Success!");
// }

fn var1() {
    let x: i32 = 5;
    let _y: i32;
    assert_eq!(x, 5);
    println!("Var 1 -> Success!");
}

// Fill the blanks in the code to make it compile
// fn main() {
//     let __ __ = 1;
//     __ += 2;

//     assert_eq!(x, 3);
//     println!("Success!");
// }

// use mut to mark the variable as mutable
fn var2() {
    let mut x: i32 = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Var 2 -> Success!");
}

// Fix the error below with least amount of modification
// fn main() {
//     let x: i32 = 10;
//     {
//         let y: i32 = 5;
//         println!("Inner scope value of x is {} and value of y is {}", x, y);
//     }
//     println!("Outer scope value of x is {} and value of y is {}", x, y);
// }

fn var3() {
    let x: i32 = 10;
    let y: i32 = 5;
    {
        println!(
            "Var 3 -> Inner scope value of x is {} and value of y is {}",
            x, y
        );
    }
    println!(
        "Var 3 -> Outer scope value of x is {} and value of y is {}",
        x, y
    );
}

// // Fix the error with the use of define_x
// fn main() {
//     println!("{}, world", x);
// }

// fn define_x() {
//     let x = "hello";
// }

fn var4() {
    let x: String = define_x();
    println!("Var 4 -> {}, world", x);
}

fn define_x() -> String {
    return "hello".to_string();
}

// // Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
// fn main() {
//     let x: i32 = 5;
//     {
//         let x = 12;
//         assert_eq!(x, 5);
//     }

//     assert_eq!(x, 12);

//     let x = 42;
//     println!("{}", x); // Prints "42".
// }

fn var5() {
    let mut x: i32 = 5;
    {
        let x: i32 = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    x = 42;
    println!("Var 5 -> {}", x);
}

// // Fix the error below with least amount of modification
// fn main() {
//     let (x, y) = (1, 2);
//     x += 2;

//     assert_eq!(x, 3);
//     assert_eq!(y, 2);

//     println!("Success!");
// }

// Fix the error below with least amount of modification
fn var6() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Var 6 -> Success!");
}

// fn main() {
//     let (x, y);
//     (x,..) = (3, 4);
//     [.., y] = [1, 2];
//     // Fill the blank to make the code work
//     assert_eq!([x,y], __);

//     println!("Success!");
// }

fn var7() {
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x, y], [3, 2]);

    println!("Var 7 -> Success!");
}
pub fn use_vars() {
    var1();
    var2();
    var3();
    var4();
    var5();
    var6();
    var7();
}
