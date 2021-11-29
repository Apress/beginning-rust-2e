/* ILLEGAL. The compiler prints twice the error message:
mismatched types
*/
fn main() {
    let mut x = ["a", "b", "c"];
    x = ["X", "Y"];
    x = [15, 16, 17];
}
