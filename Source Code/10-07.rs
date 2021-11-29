/* ILLEGAL. The compiler prints three times the error message:
mismatched types
one with the explanation:
expected `i16`, found `u16`
and the others with the explanation:
expected `i32`, found `i16`
*/
fn main() {
    fn f<T>(a: T, _b: T) -> T { a }
    let _a = f(12u8, 13u8);
    let _b = f(12i64, 13i64);
    let _c = f(12i16, 13u16);
    let _d: i32 = f(12i16, 13i16);
}
