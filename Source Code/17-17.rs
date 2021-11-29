/* ILLEGAL. The compiler prints twice the error message:
mismatched types
the first one with the explanation:
expected slice `[u8]`, found `str`
and the second one with the explanation:
expected `&[u8]`, found struct `String`
*/
fn main() {
    use std::io::Write;
    std::io::stdout().write("Hi").unwrap();
    std::io::stdout().write(String::from("Hi")).unwrap();
    std::io::stdout().write("Hello ".as_bytes()).unwrap();
    std::io::stdout().write(String::from("world").as_bytes()).unwrap();
}
