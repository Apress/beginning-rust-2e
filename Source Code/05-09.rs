/* ILLEGAL. The compiler prints the error message:
this operation will panic at runtime
with the explanation:
index out of bounds: the length is 1 but the index is 1
and with the note:
`#[deny(unconditional_panic)]` on by default
*/
fn main() {
    let x = ["a"];
    let _y = x[1];
    print!("End");
}
