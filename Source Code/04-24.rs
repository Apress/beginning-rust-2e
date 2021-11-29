/* It prints:
1 0 */
fn main() {
    let mut _i = 1;
    if true { let _i = 2; }
    print!("{} ", _i);
    while _i > 0 { _i -= 1; let _i = 5; }
    print!("{} ", _i);
}
