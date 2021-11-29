/* ILLEGAL. The compiler prints the two error messages:
cannot add `char` to `char`
cannot add `bool` to `bool`
*/
fn main() {
    let _a = 'a' + 'b';
    let _b = false + true;    
}
