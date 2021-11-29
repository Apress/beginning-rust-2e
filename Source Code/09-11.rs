/* ILLEGAL. The compiler prints twice the error message:
mismatched types
once with the explanation:
expected `i16`, found floating-point number
and once with the explanation:
expected `i16`, found `u16`
*/
fn main() {
    fn f(a: i16) {}
    f(3.);
    f(3u16);
    f(3i16);
    f(3);
}
