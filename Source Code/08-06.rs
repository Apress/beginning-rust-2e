/* ILLEGAL. The compiler prints the error message:
mismatched types
with the explanation:
expected `bool`, found `char`
*/
fn main() {
    let data1 = (10, 'x', 12, 183.19, 'Q', false, -9);
    let mut data2: (u16, char, i16, f64, bool, char, i16);
    data2 = data1;
}
