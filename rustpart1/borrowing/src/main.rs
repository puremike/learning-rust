mod borrow;
mod string_vs_str;
fn main() {
    println!("Understanding the concept of Borrowing...\n");
    borrow::use_borrow();

    println!("Understanding the concept of String vs &str ...\n");
    string_vs_str::use_stringvsstr();
}
