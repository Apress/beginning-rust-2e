/* ILLEGAL. The compiler prints the error message:
missing lifetime specifier
*/
fn main() {
    struct S {
        b: bool,
        ri: & i32,
    }
    let x: i32 = 12;
    let y: S = S { b: true, ri: &x };
    print!("{} {}", y.b, *y.ri);
}
