/* ILLEGAL. The compiler prints the error message:
no method named `clone` found for struct `S` in the current scope
*/
fn main() {
    struct S {}
    let s = S {};
    let _ = s.clone();
}
