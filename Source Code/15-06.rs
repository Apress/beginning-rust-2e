/* ILLEGAL. The compiler prints the error message:
literal out of range for `u8`
*/
fn main() {
    let _r = 3u8..1200;
}
