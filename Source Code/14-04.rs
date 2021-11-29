/* In a 64-bit system, it prints:
16 16 16; 8 8 8
In a 32-bit system, it prints:
8 8 8; 4 4 4
*/
fn main() {
    use std::mem::*;
    let a: &str = "";
    let b: &str = "0123456789";
    let c: &str = "abcdè";
    print!("{} {} {}; ",
        size_of_val(&a),
        size_of_val(&b),
        size_of_val(&c));
    print!("{} {} {}",
        size_of_val(&&a),
        size_of_val(&&b),
        size_of_val(&&c));
}
