/* ILLEGAL. The compiler prints the error message:
mismatched types
with the explanation:
expected `()`, found integer
*/
fn main() {
    let _: () = 4 / 3;
}
