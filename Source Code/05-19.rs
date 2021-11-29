/* ILLEGAL. The compiler prints the error message:
attempt to use a non-constant value in a constant
*/
fn main() {
    let length = 6;
    let arr = [0; length];
}
