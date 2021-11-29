/* ILLEGAL. The compiler prints the error message:
mismatched types
with the explanation:
expected `()`, found `u32`
*/
fn main() {
    let _: () = 4u32 / 3u32;
}
