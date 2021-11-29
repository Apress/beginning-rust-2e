/* ILLEGAL. The compiler prints the error message:
attempt to use a non-constant value in a constant
*/
fn main() {
    let n = 20;
    let _ = [0; n];
}
