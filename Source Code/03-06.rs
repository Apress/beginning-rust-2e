/* The compiler prints the warning message:
variable does not need to be mutable
Then, the program prints:
12
*/
fn main() {
    let mut number = 12;
    println!("{}", number);
}
