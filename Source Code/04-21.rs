/* ILLEGAL. The compiler prints the error message:
cannot find value `n` in this scope
*/
fn main() {
    { let n = 10; }
    print!("{} ", n);
}
