/* It prints:
13 true 12*/
fn main() {
    fn f<'a, 'b>(x: &'a i32, y: &'b i32) -> (&'b i32, bool, &'a i32) {
        (y, true, x)
    }
    let i = 12;
    let j = 13;
    let r = f(&i, &j);
    print!("{} {} {}", *r.0, r.1, *r.2);
}
