/* It prints:
12*/
fn main() {
    let x: i32 = 12;
    let y: &i32 = &x;
    print!("{}", *y);
}
