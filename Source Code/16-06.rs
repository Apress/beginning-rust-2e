/* ILLEGAL. The compiler prints the two error messages:
no method named `next` found for struct `RangeTo` in the current scope
no method named `next` found for struct `RangeFull` in the current scope
*/
fn main() {
    let _v1 = (0u32..10).next();
    let _v2 = (5u32..).next();
    let _v3 = (..8u32).next(); // ILLEGAL: Missing start value
    let _v4 = (..).next(); // ILLEGAL: Missing start value
}
