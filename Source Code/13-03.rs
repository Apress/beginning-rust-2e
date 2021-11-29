/* ILLEGAL. The compiler prints the error message:
can't capture dynamic environment in a fn item
*/
fn main() {
    let two = 2.;
    fn print_double(x: f64) {
        print!("{}", x * two);
    }
    print_double(17.2);
}
