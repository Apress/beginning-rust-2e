/* ILLEGAL. The compiler prints the error message:
cannot find function `to_string` in this scope
*/
fn main() {
    print!("{} {}",
        "abcd".to_string(),
        to_string("abcd"));
}
