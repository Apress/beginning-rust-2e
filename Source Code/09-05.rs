/* ILLEGAL. The compiler prints the error message:
the name `f` is defined multiple times
*/
fn main() {
    fn f() {}
    fn f() {}
}
