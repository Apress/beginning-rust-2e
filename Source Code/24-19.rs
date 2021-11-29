/* It prints:
13*/
fn main() {
    fn f<'a>(b: &'a mut u8) -> &'a u8 {
        *b += 1;
        b
    }
    let mut byte = 12u8;
    let byte_ref = f(&mut byte);
    print!("{}", *byte_ref);
}
