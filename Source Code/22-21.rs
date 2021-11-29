/* ILLEGAL. The compiler prints the error message:
no method named `clone` found for struct `File` in the current scope
*/
fn main() {
    let a3 = std::fs::File::open(".").unwrap();
    let b3 = a3.clone();
    let c3 = a3;
    print!(" {:?}", a3);
    print!(" {:?}", c3);
}
