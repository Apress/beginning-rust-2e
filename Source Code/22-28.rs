/* ILLEGAL. The compiler prints the error message:
the trait `Copy` may not be implemented for this type
*/
fn main() {
    struct S { x: Vec<i32> }
    impl Copy for S {}
    impl Clone for S {
        fn clone(&self) -> Self { *self }
    }
}
