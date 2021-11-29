/* ILLEGAL. The compiler prints the two error messages:
mismatched types
cannot add `i16` to `i8`
*/
fn main() {
    let a: i8 = 5;
    let b: i16 = 5;
    print!("{}", a + b);
}
