/* ILLEGAL. The compiler prints the error message:
`if` and `else` have incompatible types
*/
fn main() {
    let cond = true;
    let val = if cond { "abc" } else { 12 };
}
