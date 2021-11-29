/* ILLEGAL. The compiler prints four times the error message:
mismatched types
once with the explanation:
expected `i32`, found `bool`
and the other times with the explanation:
expected `i32`, found `()`
*/
fn main() {
    fn _f1() -> i32 { 4.5; "abc"; false }
    fn _f2() -> i32 { 4.5; "abc"; () }
    fn _f3() -> i32 { 4.5; "abc"; {} }
    fn _f4() -> i32 { 4.5; "abc"; }
}
