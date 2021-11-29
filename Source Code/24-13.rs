/* ILLEGAL. The compiler prints the error message:
missing lifetime specifier
*/
// In some library code:
struct S {
    b: bool,
    ri: & i32,
}
fn create_s(ri: &i32) -> S {
    S { b: true, ri: ri }
}
// In application code:
fn main() {
    let y: S;
    {
        let x: i32 = 12;
        y = create_s(&x);
    }
    print!("{} {}", y.b, *y.ri);
}
