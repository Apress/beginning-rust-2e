/* It prints:
abc abc abc
*/
fn main() {
    let s1: String = "abc".to_string();
    let s2: &String = &s1;
    let s3: &str = &s1;
    println!("{} {} {}", s1, s2, s3);
}
