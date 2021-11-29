/* ILLEGAL. The compiler prints the error message:
borrow of moved value: `a3`
*/
fn main() {
    let a3 = std::fs::File::open(".").unwrap();
    let c3 = a3;
    print!(" {:?}", a3);
    print!(" {:?}", c3);
}
