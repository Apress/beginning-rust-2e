/* ILLEGAL. The compiler prints the error message:
this operation will panic at runtime
with the explanation:
index out of bounds: the length is 1 but the index is 1
*/
fn main() {
    let x = ["a"]; // array of strings
    let _y = x[1];
}
