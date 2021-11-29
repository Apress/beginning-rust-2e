/* It prints:
4 4*/
fn main() {
    let s1 = "abcd".to_string();
    let s2 = s1.clone();
    let s3 = s1;
    // ILLEGAL: print!("{} ", s1.len());
    print!("{} {}", s2.len(), s3.len());
}
