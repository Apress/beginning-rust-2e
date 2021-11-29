/* ILLEGAL. The compiler prints the error message:
use of moved value: `s`
*/
fn main() {
    struct S {}
    impl Clone for S {
        fn clone(&self) -> Self { Self {} }
    }
    let s = S {};
    let _ = s.clone();
    let _s2 = s;
    let _s3 = s;
}
