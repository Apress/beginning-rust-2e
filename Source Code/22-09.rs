/* ILLEGAL. The compiler prints the error message:
borrow of moved value: `s1`
*/
fn main() {
    let s1 = "abcd".to_string();
    let s2 = s1.clone();
    let s3 = s1;
    print!("{} ", s1.len());
    print!("{} {}", s2.len(), s3.len());
}
