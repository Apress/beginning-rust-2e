/* ILLEGAL. The compiler prints the error message:
`if` may be missing an `else` clause
*/
fn main() {
    let cond = true;
    let val = if cond { "abc" };
}
