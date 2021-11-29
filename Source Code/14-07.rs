/* It prints:
8 1; 8 2; 8 3*/
fn main() {
    let mut s1 = "".to_string();
    s1.push('e');
    let mut s2 = "".to_string();
    s2.push('è');
    let mut s3 = "".to_string();
    s3.push('€');
    print!("{} {}; ", s1.capacity(), s1.len());
    print!("{} {}; ", s2.capacity(), s2.len());
    print!("{} {}", s3.capacity(), s3.len());
}
