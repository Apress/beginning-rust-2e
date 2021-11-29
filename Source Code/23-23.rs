/* ILLEGAL. The compiler prints twice the error message:
explicit lifetime required in the type of `y`
*/
fn main() {
    fn f<'a>(x: &'a Vec<u8>, y: &Vec<u8>) -> &'a u8 {
        if true { &x[0] } else { &y[0] }}
}
