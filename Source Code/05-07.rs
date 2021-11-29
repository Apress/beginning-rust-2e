/* ILLEGAL. The compiler prints the error message:
unused variable: `x`
and then it prints the warning message:
unused variable: `y`
*/
fn main() {
    #[deny(unused_variables)]
    let x = 1;
    #[warn(unused_variables)]
    let y = 2;
    #[allow(unused_variables)]
    let z = 3;
}
