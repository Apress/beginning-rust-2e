/* ILLEGAL. The compiler prints the error message:
borrow of possibly-uninitialized variable: `number1`
*/
fn main() {
    let number1;
    print!("{}", number1);
    number1 = 12;
}
