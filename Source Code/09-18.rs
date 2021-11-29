/* It prints:
4 0*/
fn main() {
    fn f(x: f64) -> f64 {
        if x <= 0. { return 0.; }
        return x + 3.;
    }
    print!("{} {}", f(1.), f(-1.));
}
