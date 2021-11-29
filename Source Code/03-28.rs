/* ILLEGAL. The compiler prints the error message:
cannot assign twice to immutable variable `_n`
*/
fn main() {
    let mut _n = 1;
    _n = 2;
    let _n = 3.14;
    _n = 5.9;
}
