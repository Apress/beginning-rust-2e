/* ILLEGAL. The compiler prints the error message:
no field `i` on type `({integer}, {integer}, {integer})`
*/
fn main() {
    let array = [12, 13, 14];
    let tuple = (12, 13, 14);
    let i = 0;
    print!("{}", array[i]);
    print!("{}", tuple.i);
}
