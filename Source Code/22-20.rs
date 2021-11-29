/* ILLEGAL. The compiler prints the error message:
borrow of moved value: `b2`
*/
fn main() {
    let a2 = Vec::<bool>::new();
    let b2 = a2.clone();
    let c2 = b2;
    print!(" {:?}", a2);
    print!(" {:?}", b2);
    print!(" {:?}", c2);
}
