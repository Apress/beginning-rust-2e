/* ILLEGAL. The compiler prints the error message:
mismatched types
with the explanation:
expected `i16`, found `u16`
*/
fn main() {
    let i = 0;
    let _j: u16 = i;
    let _k: i16 = i;
}
