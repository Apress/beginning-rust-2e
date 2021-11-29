/* ILLEGAL. The compiler prints the error message:
mismatched types
with the explanation:
expected `u32`, found `i32`
*/
fn main() {
    fn f() -> i32 { 3 }
    let _a: u32 = f();
}
