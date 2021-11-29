/* ILLEGAL. The compiler prints the error message:
the trait bound `S: Clone` is not satisfied
*/
fn main() {
    struct S {}
    impl Copy for S {}
}
