/* It prints:
8 4*/
fn main() {
    fn print_double(mut x: f64) {
        x *= 2.;
        print!("{}", x);
    }
    let x = 4.;
    print_double(x);
    print!(" {}", x);
}
