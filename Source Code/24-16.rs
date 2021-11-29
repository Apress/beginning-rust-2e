/* ILLEGAL. The compiler prints the error message:
`x` does not live long enough
*/
// In some library code:
struct S<'a> { b: bool, ri: &'a i32 }
fn create_s<'b>(ri: &'b i32) -> S<'b> {
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
