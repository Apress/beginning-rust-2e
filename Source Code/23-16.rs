/* It prints:
true 4 Hello 3.14*/
fn main() {
    static FOUR: u8 = 4;
    fn f() -> (bool, &'static u8, &'static str, &'static f64) {
        (true, &FOUR, "Hello", &3.14)
    }
    print!("{} {} {} {}", f().0, *f().1, f().2, *f().3);
}
