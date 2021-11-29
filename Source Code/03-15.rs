/* ILLEGAL. The compiler prints the error messages:
destructuring assignments are unstable
and
in expressions, `_` can only be used on the left-hand side of an assignment
*/
fn main() {
    let _ = 12;
    print!("{}", _);
}
