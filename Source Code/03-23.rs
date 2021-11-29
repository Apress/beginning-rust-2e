/* ILLEGAL. The compiler prints the error message:
mismatched types
with the explanation:
expected integer, found floating-point number
*/
fn main() {
    let mut n = 1;
    print!("{}", n);
    n = 2;
    print!(" {}", n);
    n = 3.14;
    print!(" {}", n);
}
