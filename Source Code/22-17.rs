/* ILLEGAL. The compiler prints the error message:
use of moved value: `s1`
*/
fn main() {
    struct S {}
    let s1 = S {};
    let _s2 = s1;
    s1;
}
