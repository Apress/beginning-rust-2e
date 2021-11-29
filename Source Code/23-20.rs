/* It prints:
12 true 13*/
fn main() {
    fn f<'a>(x: &'a i32, y: &'a i32) -> (&'a i32, bool, &'a i32) {
        (x, true, y)
    }
    let i = 12;
    let j = 13;
    let r = f(&i, &j);
    print!("{} {} {}", *r.0, r.1, *r.2);
}
