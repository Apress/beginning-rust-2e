/* ILLEGAL. The compiler prints the error message:
cannot define inherent `impl` for a type outside of the crate where the type is defined
*/
fn main() {
    impl Vec<i32> { }
}
