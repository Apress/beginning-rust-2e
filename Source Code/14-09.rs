/* It prints:
()*/
fn main() {
    let s1 = String::new();
    let s2 = String::from("");
    let s3 = "".to_string();
    let s4 = "".to_owned();
    let s5 = format!("");
    print!("({}{}{}{}{})", s1, s2, s3, s4, s5);
}
