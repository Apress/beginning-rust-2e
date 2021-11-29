/* ILLEGAL. The compiler prints twice the error message:
mismatched types
For the first one, the explanation is:
expected `i32`, found enum `T`
For the second one, the explanation is:
expected enum `T`, found integer
*/
fn main() {
    enum T {A, B, C, D}
    let n: i32 = T::D;
    let e: T = 1;
}
