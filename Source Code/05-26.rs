/* ILLEGAL. The compiler prints twice the error message:
mismatched types
*/
fn main() {
    let mut x = vec!["This", "is", "a", "sentence"];
    x.insert("line", 1);
}
