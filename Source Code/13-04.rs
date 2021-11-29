/* It prints:
34.4*/
fn main() {
    const TWO: f64 = 2.;
    fn print_double(x: f64) {
        print!("{}", x * TWO);
    }
    print_double(17.2);
}
