/* ILLEGAL. The compiler prints twice the error message:
lifetime mismatch
*/
fn main() {
    fn f<'a, 'b>(x: &'a i32, y: &'b i32) -> (&'b i32, bool, &'a i32) {
        (x, true, y)
    }
    let i = 12;
    let j = 13;
    let r = f(&i, &j);
    print!("{} {} {}", *r.0, r.1, *r.2);
}
