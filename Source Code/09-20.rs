/* It prints:
5*/
fn main() {
    fn f(x: i32) {
        if x <= 0 { return; }
        if x == 4 { return (); }
        if x == 7 { return {}; }
        print!("{}", x);
    }
    f(5);
}
