/* It prints:
12345 12345*/
fn main() {
    let i1 = Box::new(12345i16);
    let i2 = i1.clone();
    let i3 = i1;
    // ILLEGAL: print!("{} ", i1);
    print!("{} {}", i2, i3);
}
