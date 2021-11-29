/* It prints:
true false; false true*/
fn main() {
    let mut a = Some(12);
    print!("{} {}; ", a.is_some(), a.is_none());
    a = None;
    print!("{} {}", a.is_some(), a.is_none());
}
