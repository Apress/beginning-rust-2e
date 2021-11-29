/* ILLEGAL. The compiler prints the error message:
mismatched types
*/
fn main() {
    let mut _x = vec!["a", "b", "c"];
    _x = vec![15, 16, 17];
}
