/* ILLEGAL. The compiler prints the three error messages:
mismatched types
cannot apply unary operator `-` to type `u32`
mismatched types
*/
fn main() {
    let r1 = 3u8..12i8;
    let r2: std::ops::Range<u32> = -3..12;
    let r3: std::ops::Range<i32> = 3i16..12;
}
