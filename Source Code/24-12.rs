/* ILLEGAL. The compiler prints the error message:
missing lifetime specifier
*/
fn main() {
    struct S {
        b: bool,
        ri: & i32,
    }
    let y: S;
    {
        let x: i32 = 12;
        y = S { b: true, ri: &x };
    }
    print!("{} {}", y.b, *y.ri);
}
